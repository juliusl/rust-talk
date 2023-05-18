use crate::*;

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