use crate::HashMap;
use std::{borrow::Borrow, hash::Hash, ops::Index};

impl<K, Q, V> Index<&Q> for HashMap<K, V>
where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}
