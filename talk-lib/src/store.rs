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
