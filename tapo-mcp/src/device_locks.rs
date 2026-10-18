use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::{OwnedMutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Makes tool calls that reach the same device IP take turns, and keeps them apart
/// from background discovery runs, which log in to every device.
#[derive(Default)]
pub struct DeviceLocks {
    /// Held for writing by each background discovery run, and for reading by
    /// every tool call that contacts a device.
    discovery: RwLock<()>,
    // A std lock is fine: it is never held across an `.await`.
    ips: Mutex<HashMap<String, IpEntry>>,
}

struct IpEntry {
    lock: Arc<tokio::sync::Mutex<()>>,
    /// Calls that hold or wait for `lock`. The entry is removed when this reaches 0.
    users: usize,
}

/// Counts one call as a user of an IP's entry from before it waits until it is
/// dropped, so the entry is removed even if the call is cancelled while waiting.
struct IpUser<'a> {
    locks: &'a DeviceLocks,
    ip: String,
}

impl Drop for IpUser<'_> {
    fn drop(&mut self) {
        let mut ips = self.locks.ips.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(entry) = ips.get_mut(&self.ip) {
            entry.users -= 1;
            if entry.users == 0 {
                ips.remove(&self.ip);
            }
        }
    }
}

/// Held for the whole of one tool call's contact with a device.
pub(crate) struct DeviceGuard<'a> {
    _ip: OwnedMutexGuard<()>,
    _user: IpUser<'a>,
    _discovery: RwLockReadGuard<'a, ()>,
}

impl DeviceLocks {
    /// Waits for the other calls to `ip`, then for any discovery run in progress.
    ///
    /// The IP lock is taken first, so a call waiting behind others on the same IP
    /// does not hold back a discovery run.
    pub(crate) async fn lock_device(&self, ip: &str) -> DeviceGuard<'_> {
        let (user, ip_lock) = {
            let mut ips = self.ips.lock().unwrap_or_else(|e| e.into_inner());
            let entry = ips.entry(ip.to_string()).or_insert_with(|| IpEntry {
                lock: Default::default(),
                users: 0,
            });
            entry.users += 1;
            let user = IpUser {
                locks: self,
                ip: ip.to_string(),
            };
            (user, Arc::clone(&entry.lock))
        };
        let ip_guard = ip_lock.lock_owned().await;
        let discovery = self.discovery.read().await;
        DeviceGuard {
            _ip: ip_guard,
            _user: user,
            _discovery: discovery,
        }
    }

    /// Waits for the tool calls in progress, then holds back new ones until dropped.
    pub(crate) async fn lock_discovery(&self) -> RwLockWriteGuard<'_, ()> {
        self.discovery.write().await
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::timeout;

    use super::*;

    const WAIT: Duration = Duration::from_millis(50);

    #[tokio::test]
    async fn same_ip_waits() {
        let locks = DeviceLocks::default();
        let guard = locks.lock_device("192.168.1.10").await;

        assert!(
            timeout(WAIT, locks.lock_device("192.168.1.10"))
                .await
                .is_err()
        );

        drop(guard);
        assert!(
            timeout(WAIT, locks.lock_device("192.168.1.10"))
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn last_guard_removes_ip() {
        let locks = DeviceLocks::default();
        let first = locks.lock_device("192.168.1.10").await;
        let mut second = Box::pin(locks.lock_device("192.168.1.10"));
        assert!(timeout(WAIT, &mut second).await.is_err());

        drop(first);
        let second = second.await;
        assert_eq!(locks.ips.lock().unwrap().len(), 1);

        drop(second);
        assert!(locks.ips.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn cancelled_waiter_removes_ip() {
        let locks = DeviceLocks::default();
        let first = locks.lock_device("192.168.1.10").await;
        assert!(
            timeout(WAIT, locks.lock_device("192.168.1.10"))
                .await
                .is_err()
        );

        drop(first);
        assert!(locks.ips.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn different_ip_does_not_wait() {
        let locks = DeviceLocks::default();
        let _guard = locks.lock_device("192.168.1.10").await;

        assert!(
            timeout(WAIT, locks.lock_device("192.168.1.11"))
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn device_waits_for_discovery() {
        let locks = DeviceLocks::default();
        let guard = locks.lock_discovery().await;

        assert!(
            timeout(WAIT, locks.lock_device("192.168.1.10"))
                .await
                .is_err()
        );

        drop(guard);
        assert!(
            timeout(WAIT, locks.lock_device("192.168.1.10"))
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn discovery_waits_for_device() {
        let locks = DeviceLocks::default();
        let guard = locks.lock_device("192.168.1.10").await;

        assert!(timeout(WAIT, locks.lock_discovery()).await.is_err());

        drop(guard);
        assert!(timeout(WAIT, locks.lock_discovery()).await.is_ok());
    }
}
