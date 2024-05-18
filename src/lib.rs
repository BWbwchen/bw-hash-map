use log::info;
use std::{
    borrow::Borrow,
    hash::{DefaultHasher, Hash, Hasher},
    mem::replace,
};

pub mod index;
pub mod iter;

const DEFAULT_BUCKET_LEN: usize = 4;

pub struct HashMap<K, V> {
    storage: Vec<Vec<(K, V)>>,
    // Number of key in the HashMap
    count: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash,
{
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            count: 0,
        }
    }

    fn get_ht<Q>(&self, k: Q) -> Option<usize>
    where
        Q: Hash,
    {
        if self.storage.is_empty() {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        assert!(self.storage.len() > 0);
        Some((hasher.finish() % self.storage.len() as u64) as usize)
    }

    /// resize the internal storage with strategy:
    /// - When the number of key is larger than 3/4 of the total buckets, double the number of buckets.
    /// - When the number of key is smaller than 1/4 of the total buckets, shrink the number of buckets half.
    fn resize(&mut self) {
        if self.count >= 3 * self.storage.len() / 4 {
            info!("Expand the storage size.");
            // double the number of buckets.
            let new_len = std::cmp::max(self.storage.len() * 2, DEFAULT_BUCKET_LEN);
            let mut new_storage = Vec::with_capacity(new_len);
            for _ in 0..new_len {
                new_storage.push(Vec::new());
            }

            for (k, v) in self.storage.iter_mut().flat_map(|bucket| bucket.drain(..)) {
                let mut hasher = DefaultHasher::new();
                k.hash(&mut hasher);
                let index = (hasher.finish() % new_len as u64) as usize;

                new_storage[index].push((k, v));
            }

            let _ = replace(&mut self.storage, new_storage);
        } else if self.count <= self.storage.len() / 4 {
            // shrink the number of buckets.
            info!("Shrink the storage size.");
            // TODO: currently, we do not shrink.
        } else {
            // do nothing
        }
    }

    /// return:
    /// - `None`, if this key isn't exist in the map.
    /// - v, if this key is exist in the map, the corresponding value is updated and return the old valud.
    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Hash + Eq,
    {
        self.resize();
        let idx = self.get_ht(&k)?;
        for (vk, vv) in self.storage[idx].iter_mut() {
            if *vk == k {
                return Some(replace(vv, v));
            }
        }
        self.storage[idx].push((k, v));
        self.count += 1;
        None
    }

    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized + Eq,
    {
        self.get(k).is_some()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    /// remove the key entry from the map, return:
    /// - `None`, if this key isn't exist in the map.
    /// - Some(v), if remove successfully.
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized + Eq,
    {
        let idx = self.get_ht(k)?;
        let hm_idx = self
            .storage
            .get(idx)?
            .iter()
            .position(|e| e.0.borrow() == k)?;
        self.count -= 1;
        let ret = self.storage[idx].swap_remove(hm_idx).1;
        self.resize();
        Some(ret)
    }

    /// get the value with given key from the map, return:
    /// - `None`, if this key isn't exist in the map.
    /// - Some(v).
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized + Eq,
    {
        let idx = self.get_ht(k)?;
        self.storage
            .get(idx)?
            .iter()
            .find(|e| e.0.borrow() == k)
            .map(|hme| hme.1.borrow())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hasher() {
        let hm: HashMap<String, String> = HashMap::new();
        assert_eq!(hm.get_ht("Hello"), None);
    }

    #[test]
    fn basic_insert() {
        let mut hm = HashMap::new();
        hm.insert(3, 3);
        assert_eq!(hm.get(&3), Some(&3));
        assert!(hm.contains_key(&3));
        hm.insert(4, 4);
        assert_eq!(hm.get(&4), Some(&4));
        assert!(hm.contains_key(&4));
        hm.insert(5, 5);
        assert_eq!(hm.get(&5), Some(&5));
        assert!(hm.contains_key(&5));
        hm.insert(6, 6);
        assert_eq!(hm.get(&6), Some(&6));
        assert!(hm.contains_key(&6));
    }

    // From std documentation example
    #[test]
    fn basic_functionality() {
        let mut book_reviews = HashMap::new();

        // Review some books.
        book_reviews.insert(
            "Adventures of Huckleberry Finn".to_string(),
            "My favorite book.".to_string(),
        );
        book_reviews.insert(
            "Grimms' Fairy Tales".to_string(),
            "Masterpiece.".to_string(),
        );
        book_reviews.insert(
            "Pride and Prejudice".to_string(),
            "Very enjoyable.".to_string(),
        );
        book_reviews.insert(
            "The Adventures of Sherlock Holmes".to_string(),
            "Eye lyked it alot.".to_string(),
        );

        assert!(book_reviews.contains_key("Adventures of Huckleberry Finn"));
        assert!(book_reviews.contains_key("Grimms' Fairy Tales"));
        assert!(book_reviews.contains_key("Pride and Prejudice"));
        assert!(book_reviews.contains_key("The Adventures of Sherlock Holmes"));
        assert!(!book_reviews.contains_key("Les Misérables"));

        assert_eq!(book_reviews.len(), 4);

        // oops, this review has a lot of spelling mistakes, let's delete it.
        book_reviews.remove("The Adventures of Sherlock Holmes");
        assert!(!book_reviews.contains_key("The Adventures of Sherlock Holmes"));
        assert_eq!(book_reviews.len(), 3);

        let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
        let to_find_answer_is_some = [true, false];
        for (id, &book) in to_find.iter().enumerate() {
            assert_eq!(book_reviews.get(book).is_some(), to_find_answer_is_some[id]);
        }

        // Look up the value for a key (will panic if the key is not found).
        assert_eq!(
            book_reviews["Pride and Prejudice"],
            "Very enjoyable.".to_string()
        );

        let review_answer = [
            ("Adventures of Huckleberry Finn", "My favorite book."),
            ("Grimms' Fairy Tales", "Masterpiece."),
            ("Pride and Prejudice", "Very enjoyable."),
        ];

        for (key, review) in &book_reviews {
            for (k, v) in &review_answer {
                if key == k {
                    assert_eq!(review, v);
                }
            }
        }

        for (key, review) in book_reviews.into_iter() {
            for (k, v) in review_answer {
                if key == *k {
                    assert_eq!(review, *v);
                }
            }
        }
    }

    #[test]
    fn test_insert_update() {
        let mut hm = HashMap::new();

        hm.insert(3, 3);
        assert_eq!(hm.get(&3), Some(&3));
        hm.insert(3, 4);
        assert_eq!(hm.get(&3), Some(&4));

        let mut hm = HashMap::new();

        hm.insert("Hello", 3);
        assert_eq!(hm.get("Hello"), Some(&3));
        hm.insert("Hello", 4);
        assert_eq!(hm.get("Hello"), Some(&4));
    }
}
