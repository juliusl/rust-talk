use crate::{Item, Error};

pub mod vec;

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

/// Super-trait representing storage,
/// 
pub trait Storage<T, Index> : FindMut<T, Index = Index> + Find<T, Index = Index> + RemoveByIndex<T, Index = Index> + InsertItem<T, Index = Index> 
where
    for<'a> T: Item<'a>
{
}
