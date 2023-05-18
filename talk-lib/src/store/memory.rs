// use talk_derive::Store;
// use crate::BaseItem;
use crate::Error;
use crate::store::*;

/// Memory implementation for some store T
///
pub struct MemoryStore<T>
where
    for<'a> T: Item<'a>,
{
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

// Naive exmaple
impl<T> Replace<T> for MemoryStore<T>
where
    for<'a> T: Item<'a>,
{
    fn replace(&mut self, item: &T) -> Result<&T, crate::Error> {
        let (pk, registry_id) = item.as_ref().get_row_id();

        if let Some((loc, _item)) = self.items.iter_mut().enumerate().find(|(_, i)| {
            match i.as_ref().get_row_id() {
                (ref _p, ref _rid) if pk == *_p && registry_id == *_rid  => true,
                _ => false
            }
        }) {
            *_item = item.clone();
            Ok(self.items.get(loc).expect("should exist jsut added"))
        } else {
            Err(Error::new())
        }
    }
}

impl<T> Create<T> for MemoryStore<T>
where
    for<'a> T: Item<'a>,
{
    fn create(&mut self, item: &T) -> Result<&T, Error> {
        let mut cloned = item.clone();
        // Naive example
        cloned.as_mut().set_row_id(
            "test", 
            format!("item-{}", self.items.len())
        );


        self.items.push(cloned);
        Ok(self.items.last().expect("should exist just added"))
    }
}

// impl<T> Read<T> for MemoryStore<T>
// where
//     for<'a> T: Item<'a>,
// {
//     fn read(&self, item: &T) -> Result<&T, Error> {
//         if let Some((_, _item)) = self.find(item) {
//             Ok(_item)
//         } else {
//             Err(Error::new())
//         }
//     }
// }

// impl<T> Replace<T> for MemoryStore<T>
// where
//     for<'a> T: Item<'a>,
// {
//     fn replace(&mut self, item: &T) -> Result<&T, crate::Error> {
//         if let Some((_, replacing)) = self.find_mut(&item) {
//             *replacing = item.clone();
//             // Ok(self.items.get(loc).expect("should exist just replaced"))
//         }
//         // } else {
//         //     Err(Error::new())
//         // }

        
//         self.items.read(item)

//         // self.items.replace(item)
//     }
// }

// impl<T> Delete<T> for MemoryStore<T>
// where
//     for<'a> T: Item<'a>,
// {
//     fn delete(&mut self, item: &T) -> Result<T, Error> {
//         // if let Some((loc, _)) = self.find_mut(item) {
//         //     let removed = self.items.remove(loc);
//         //     Ok(removed)
//         // } else {
//         //     Err(Error::new())
//         // }

//         self.items.delete(item)
//     }
// }

// impl<T> Read<T> for MemoryStore<T>
// where
//     for<'a> T: Item<'a>,
// {
//     fn read(&self, item: &T) -> Result<&T, Error> {
//         if let Some((_, _item)) = self.find(item) {
//             Ok(_item)
//         } else {
//             Err(Error::new())
//         }
//     }
// }

// impl<T> Store<T> for MemoryStore<T>
// where
//     for<'a> T: Item<'a>,
//     Self: From<StoreInfo> + AsRef<StoreInfo>,
// {
// }

// impl<T> Store<T> {
//     // /// Finds and returns a mutable reference to T and it's index,
//     // ///
//     // fn find_mut(&mut self, item: &T) -> Option<(usize, &mut T)> {
//     //     let (pk, registry_id) = item.as_ref().get_row_id();

//     //     self.items
//     //         .iter_mut()
//     //         .enumerate()
//     //         .find(|(_, i)| match i.as_ref().get_row_id() {
//     //             (ref _p, ref _rid) if pk == *_p && registry_id == *_rid => true,
//     //             _ => false,
//     //         })
//     // }

//     // /// Finds and returns a reference to T and it's index,
//     // ///
//     // fn find(&self, item: &T) -> Option<(usize, &T)> {
//     //     let (pk, registry_id) = item.as_ref().get_row_id();

//     //     self.items
//     //         .iter()
//     //         .enumerate()
//     //         .find(|(_, i)| match i.as_ref().get_row_id() {
//     //             (ref _p, ref _rid) if pk == *_p && registry_id == *_rid => true,
//     //             _ => false,
//     //         })
//     // }
// }