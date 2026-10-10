use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::Router;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{DateTime, Utc};
use rand::RngExt as _;

/// How long a snapshot link stays valid after the snapshot is taken.
pub const SNAPSHOT_TTL: Duration = Duration::from_secs(300);
const MAX_SNAPSHOTS: usize = 1024;

/// In-memory store of recent snapshots, keyed by an unguessable token.
#[derive(Default)]
pub struct SnapshotStore {
    inner: Mutex<StoreInner>,
}

#[derive(Default)]
struct StoreInner {
    entries: HashMap<String, StoredSnapshot>,
    /// Tokens in insertion order. Every snapshot gets the same TTL, so this is also expiry order.
    order: VecDeque<String>,
}

#[derive(Clone)]
struct StoredSnapshot {
    data: Bytes,
    expires_at: DateTime<Utc>,
}

impl SnapshotStore {
    /// Stores a snapshot for [`SNAPSHOT_TTL`] and returns the token it can be fetched with,
    /// along with when it expires.
    pub fn insert(&self, data: Vec<u8>) -> (String, DateTime<Utc>) {
        self.insert_at(Utc::now(), data)
    }

    fn insert_at(&self, now: DateTime<Utc>, data: Vec<u8>) -> (String, DateTime<Utc>) {
        let mut inner = self.inner.lock().unwrap_or_else(|e| e.into_inner());

        while inner
            .order
            .front()
            .and_then(|token| inner.entries.get(token))
            .is_some_and(|entry| entry.expires_at <= now)
        {
            inner.remove_oldest();
        }

        if inner.order.len() >= MAX_SNAPSHOTS {
            inner.remove_oldest();
        }

        let token = generate_token();
        let expires_at = now + SNAPSHOT_TTL;
        inner.entries.insert(
            token.clone(),
            StoredSnapshot {
                data: data.into(),
                expires_at,
            },
        );
        inner.order.push_back(token.clone());
        (token, expires_at)
    }

    fn get_at(&self, now: DateTime<Utc>, token: &str) -> Option<StoredSnapshot> {
        let inner = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        inner
            .entries
            .get(token)
            .filter(|entry| entry.expires_at > now)
            .cloned()
    }
}

impl StoreInner {
    fn remove_oldest(&mut self) {
        if let Some(token) = self.order.pop_front() {
            self.entries.remove(&token);
        }
    }
}

fn generate_token() -> String {
    let bytes: [u8; 32] = rand::rng().random();
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Routes serving stored snapshots at `/snapshots/{token}.jpg`.
///
/// Deliberately unauthenticated: a browser following the link will not send
/// the bearer token, so the token in the path is the only credential.
pub fn router(store: Arc<SnapshotStore>) -> Router {
    Router::new()
        .route("/snapshots/{file}", get(get_snapshot))
        .with_state(store)
}

async fn get_snapshot(
    State(store): State<Arc<SnapshotStore>>,
    Path(file): Path<String>,
) -> Response {
    let Some(token) = file.strip_suffix(".jpg") else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let Some(snapshot) = store.get_at(Utc::now(), token) else {
        tracing::debug!("Snapshot link miss");
        return StatusCode::NOT_FOUND.into_response();
    };
    tracing::debug!("Snapshot link hit");

    (
        [
            (header::CONTENT_TYPE, "image/jpeg"),
            (header::CACHE_CONTROL, "private, no-store"),
            (
                header::CONTENT_DISPOSITION,
                "inline; filename=\"snapshot.jpg\"",
            ),
        ],
        snapshot.data,
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_none_after_ttl() {
        let store = SnapshotStore::default();
        let now = Utc::now();
        let (token, expires_at) = store.insert_at(now, vec![1, 2, 3]);

        assert_eq!(expires_at, now + SNAPSHOT_TTL);

        assert!(
            store
                .get_at(now + SNAPSHOT_TTL - Duration::from_secs(1), &token)
                .is_some()
        );
        assert!(store.get_at(now + SNAPSHOT_TTL, &token).is_none());
    }

    #[test]
    fn insert_beyond_cap_evicts_earliest() {
        let store = SnapshotStore::default();
        let start = Utc::now();
        let tokens: Vec<String> = (0..=MAX_SNAPSHOTS)
            .map(|i| {
                store
                    .insert_at(start + Duration::from_millis(i as u64), vec![])
                    .0
            })
            .collect();

        let now = start + Duration::from_secs(1);
        assert!(store.get_at(now, &tokens[0]).is_none());
        assert!(tokens[1..].iter().all(|t| store.get_at(now, t).is_some()));
    }

    #[test]
    fn tokens_are_unique_and_43_chars() {
        let store = SnapshotStore::default();
        let (a, _) = store.insert(vec![]);
        let (b, _) = store.insert(vec![]);

        assert_ne!(a, b);
        assert_eq!(a.len(), 43);
        assert_eq!(b.len(), 43);
    }
}
