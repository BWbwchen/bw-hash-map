use std::{
    borrow::Borrow,
    hash::{DefaultHasher, Hash, Hasher},
};

struct HashMapEntry<K, V> {
    k: K,
    v: V,
}

// TODO: right now, we don't consider the resize mechanism.
const DEFAULT_NUM_BUCKET: usize = 53;

pub struct HashMap<K, V> {
    storage: Vec<Vec<HashMapEntry<K, V>>>,
    // Number of HashMapEntry in the HashMap
    count: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        let mut v = Vec::with_capacity(DEFAULT_NUM_BUCKET);
        for _ in 0..DEFAULT_NUM_BUCKET {
            v.push(Vec::new());
        }
        Self {
            storage: v,
            count: 0,
        }
    }

    fn get_ht<Q>(&self, k: Q) -> usize
    where
        Q: Hash,
    {
        // if self.storage.is_empty() {
        //     return None;
        // }
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        assert!(self.storage.len() > 0);
        (hasher.finish() % self.storage.len() as u64) as usize
    }

    /// return:
    /// - `None`, if this key isn't exist in the map.
    /// - v, if this key is exist in the map, the corresponding value is updated and return the old valud.
    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Hash,
    {
        let idx = self.get_ht(&k);
        self.storage[idx].push(HashMapEntry { k, v });
        self.count += 1;

        // TODO: suppose that there has no duplicate key.
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
        let idx = self.get_ht(k);
        if self.storage[idx].is_empty() {
            return None;
        } else {
            let hm_idx = self.storage[idx].iter().position(|e| e.k.borrow() == k)?;
            self.count -= 1;
            return Some(self.storage[idx].swap_remove(hm_idx).v);
        }
    }

    /// get the value with given key from the map, return:
    /// - `None`, if this key isn't exist in the map.
    /// - Some(v).
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized + Eq,
    {
        let idx = self.get_ht(k);
        if self.storage[idx].is_empty() {
            return None;
        } else {
            return self.storage[idx]
                .iter()
                .find(|e| e.k.borrow() == k)
                .map(|hme| hme.v.borrow());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hasher() {
        let hm: HashMap<String, String> = HashMap::new();
        let ret = hm.get_ht("Hello");
        println!("{}", ret);
    }

    // From std documentation example
    #[test]
    fn basic_functionaliy() {
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
    }
}
