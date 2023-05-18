use serde::Serialize;
use serde::Deserialize;
use crate::Item;
use crate::Error;
mod memory;
pub use memory::MemoryStore;

/// Information about a store
///
#[derive(Serialize, Deserialize)]
pub struct StoreInfo {
    /// Name of the database
    /// 
    database_name: String,
    /// Id of the container
    /// 
    container_id: String,
}

impl StoreInfo {
    /// Creates a new store info,
    ///
    pub fn new(database_name: impl Into<String>, container_id: impl Into<String>) -> Self {
        let database_name = database_name.into();
        let container_id = container_id.into();
        Self {
            database_name,
            container_id,
        }
    }
}

/// Trait that enables a type to create an item T
///
pub trait Create<T>
where
    for<'a> T: Item<'a>,
{
    /// Creates an item T and returns the current item
    ///
    fn create(&mut self, item: &T) -> Result<&T, Error>;
}

/// Trait that enables a type to replace an item T
///
pub trait Replace<T>
where
    for<'a> T: Item<'a>,
{
    /// Replaces an item T and returns the current item
    ///
    fn replace(&mut self, item: &T) -> Result<&T, Error>;
}

/// Trait that enables a type to delete an item T
///
pub trait Delete<T>
where
    for<'a> T: Item<'a>,
{
    /// Deletes an item T and returns the current item
    ///
    fn delete(&mut self, item: &T) -> Result<T, Error>;
}

/// Trait that enables a type to read an item T
///
pub trait Read<T>
where
    for<'a> T: Item<'a>,
{
    /// Creates an item T and returns the current item
    ///
    fn read(&self, item: &T) -> Result<&T, Error>;
}

/*
Ignoring batch for brevity
*/

/// Super trait of store actions,
///
pub trait Store<T>: Replace<T> + Create<T> + Delete<T> + Read<T>
where
    for<'a> T: Item<'a>,
    Self: From<StoreInfo> + AsRef<StoreInfo>,
{
}

// impl<T> Read<T> for Vec<T> 
// where
//     for<'a> T: Item<'a>
// {
//     fn read(&self, item: &T) -> Result<&T, Error> {
//         if let Some((_, t)) = self.find(item) {
//             Ok(t)
//         } else {
//             Err(Error::new())
//         }
//     }
// }

// impl<T> Create<T> for Vec<T> 
// where
//     for<'a> T: Item<'a>
// {
//     fn create(&mut self, item: &T) -> Result<&T, Error> {
//         <Self as InsertItem<T>>::insert(self, item);

//         self.read(item)
//     }
// }

// impl<T> Delete<T> for Vec<T>
// where
//     for<'a> T: Item<'a> 
// {
//     fn delete(&mut self, item: &T) -> Result<T, Error> {
//         if let Some((loc, _)) = self.find(item) {
//             Ok(self.remove_by_index(loc).expect("should exist just found it"))
//         } else {
//             Err(Error::new())
//         }
//     }
// }

// impl<T> Replace<T> for Vec<T> 
// where
//     for<'a> T: Item<'a> 
// {
//     fn replace(&mut self, item: &T) -> Result<&T, Error> {
//         if let Some((loc, _item)) = self.find_mut(item) {
//             *_item = item.clone();
//             Ok(self.get(loc).expect("should exist jsut added"))
//         } else {
//             Err(Error::new())
//         }
//     }
// }


// impl<T> Read<T> for Vec<T> 
// where
//     for<'a> T: Item<'a>
// {
//     fn read(&self, item: &T) -> Result<&T, Error> {
//         let (pk, registry_id) = item.as_ref().get_row_id();

//         if let Some((_, t)) = self
//             .iter()
//             .enumerate()
//             .find(|(_, i)| match i.as_ref().get_row_id() {
//                 (ref _p, ref _rid) if pk == *_p && registry_id == *_rid => true,
//                 _ => false,
//             }) 
//         {
//             Ok(t)
//         } else {
//             Err(Error::new())
//         }
//     }
// }

// impl<T> Delete<T> for Vec<T>
// where
//     for<'a> T: Item<'a> 
// {
//     fn delete(&mut self, item: &T) -> Result<T, Error> {
//         let (pk, registry_id) = item.as_ref().get_row_id();

//         if let Some((loc, _item)) = self.iter().enumerate().find(|(_, i)| {
//             match i.as_ref().get_row_id() {
//                 (ref _p, ref _rid) if pk == *_p && registry_id == *_rid  => true,
//                 _ => false
//             }
//         }) {
//             Ok(self.remove(loc))
//         } else {
//             Err(Error::new())
//         }
//     }
// }

// impl<T> Replace<T> for Vec<T>
// where
//     for<'a> T: Item<'a> 
// {
//     fn replace(&mut self, item: &T) -> Result<&T, Error> {
//         let (pk, registry_id) = item.as_ref().get_row_id();

//         if let Some((_, replacing)) = self.iter_mut().enumerate().find(|(_, i)| {
//             match i.as_ref().get_row_id() {
//                 (ref _p, ref _rid) if pk == *_p && registry_id == *_rid  => true,
//                 _ => false
//             }
//         }) {
//             *replacing = item.clone();
//             self.read(item)
//         } else {
//             Err(Error::new())
//         }
//     }
// }

// TODO: Segway into code gen

/// Trait for a collection of T, that can be used to find a mutable reference to T
/// 
pub trait FindMut<T>
where
    for<'a> T: Item<'a> 
{
    /// Type that maps to T
    /// 
    type Index;

    /// Finds an item and returns it's index and a mutable reference to the item
    /// 
    fn find_mut(&mut self, item: &T) -> Option<(Self::Index, &mut T)>;
}

/// Trait for a collection of T, that can be used to find a reference to T
/// 
pub trait Find<T>
where
    for<'a> T: Item<'a> 
{
    /// Type that maps to T
    /// 
    type Index;

    /// Finds an item and returns it's index and a reference to the item
    /// 
    fn find(&self, item: &T) -> Option<(Self::Index, &T)>;
}

/// Trait for a collection of T, that can be used to find a reference to T
/// 
pub trait RemoveByIndex<T>
where
    for<'a> T: Item<'a> 
{
    /// Type that maps to T
    /// 
    type Index;

    /// Removes an item by index and returns the removed item,
    /// 
    fn remove_by_index(&mut self, index: Self::Index) -> Option<T>;
}

/// Trait for inserting an item
/// 
pub trait InsertItem<T> 
where
    for<'a> T: Item<'a>
{
    type Index;

    /// Inserts an item and returns the index
    /// 
    fn insert_item(&mut self, item: &T) -> Result<Self::Index, Error>;
}

impl<T> FindMut<T> for Vec<T> 
where
    for<'a> T: Item<'a>,
{
    type Index = usize;

    fn find_mut(&mut self, item: &T) -> Option<(Self::Index, &mut T)> {
        let (pk, registry_id) = item.as_ref().get_row_id();

        self.iter_mut().enumerate().find(|(_, i)| {
            match i.as_ref().get_row_id() {
                (ref _p, ref _rid) if pk == *_p && registry_id == *_rid  => true,
                _ => false
            }
        })
    }
}

impl<T> Find<T> for Vec<T> 
where
    for<'a> T: Item<'a>
{
    type Index = usize;

    fn find(&self, item: &T) -> Option<(Self::Index, &T)> {
        let (pk, registry_id) = item.as_ref().get_row_id();

        self.iter().enumerate().find(|(_, i)| {
            match i.as_ref().get_row_id() {
                (ref _p, ref _rid) if pk == *_p && registry_id == *_rid  => true,
                _ => false
            }
        })
    }
}

impl<T> RemoveByIndex<T> for Vec<T> 
where
    for<'a> T: Item<'a>
{
    type Index = usize;

    fn remove_by_index(&mut self, loc: Self::Index) -> Option<T> {
        Some(self.remove(loc))
    }
}

impl<T> InsertItem<T> for Vec<T> 
where
    for<'a> T: Item<'a>
{
    type Index = usize;

    fn insert_item(&mut self, item: &T) -> Result<Self::Index, Error> {
        let index = self.len();
        let cloned = item.clone();
        self.push(cloned);
        Ok(index)
    }
}