/// Implemented by all Device Info Result variations.
pub trait DeviceInfoResultExt
where
    Self: Sized,
{
    fn decode(&self) -> anyhow::Result<Self>;
}
