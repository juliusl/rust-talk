use talk_derive::Store;
use crate::Error;
use crate::*;

/// Memory implementation for some store T
///
#[derive(Store)]
pub struct MemoryStore<T>
where
    for<'a> T: Item<'a>,
{
    #[storage]
    items: Vec<T>,
    store_info: StoreInfo,
}

impl<T> From<StoreInfo> for MemoryStore<T>
where
    for<'a> T: Item<'a>,
{
    fn from(value: StoreInfo) -> Self {
        Self {
            items: vec![],
            store_info: value,
        }
    }
}

impl<T> AsRef<StoreInfo> for MemoryStore<T>
where
    for<'a> T: Item<'a>,
{
    fn as_ref(&self) -> &StoreInfo {
        &self.store_info
    }
}

impl<T> MemoryStore<T>
where
    for<'a> T: Item<'a>,
{
    /// Returns an iterator over items
    /// 
    pub fn iter_items(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}
