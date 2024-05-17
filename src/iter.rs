use std::hash::Hash;

use crate::HashMap;

pub struct HashMapIntoIterator<K, V> {
    hm: HashMap<K, V>,
    bucket_index: usize,
}

impl<K, V> IntoIterator for HashMap<K, V>
where
    K: Hash + Eq,
{
    type Item = (K, V);
    type IntoIter = HashMapIntoIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HashMapIntoIterator {
            hm: self,
            bucket_index: 0,
        }
    }
}

impl<K, V> Iterator for HashMapIntoIterator<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.hm.storage.get_mut(self.bucket_index) {
                Some(bucket) => match bucket.pop() {
                    Some(x) => break Some(x),
                    None => {
                        self.bucket_index += 1;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

pub struct HashMapIterator<'a, K, V> {
    hm: &'a HashMap<K, V>,
    bucket_index: usize,
    per_bucket_index: usize,
}

impl<'a, K, V> Iterator for HashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.hm.storage.get(self.bucket_index) {
                Some(bucket) => match bucket.get(self.per_bucket_index) {
                    Some(&(ref k, ref v)) => {
                        self.per_bucket_index += 1;
                        break Some((k, v));
                    }
                    None => {
                        self.bucket_index += 1;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HashMapIterator {
            hm: self,
            bucket_index: 0,
            per_bucket_index: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // From std documentation example
    #[test]
    fn basic_iter() {
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

        let review_answer = ["My favorite book.", "Masterpiece.", "Very enjoyable."];
        let mut idx = 0;
        for (_, review) in &book_reviews {
            assert_eq!(review, review_answer[idx]);
            idx += 1;
        }

        let review_answer = ["My favorite book.", "Masterpiece.", "Very enjoyable."];
        for (idx, (_, review)) in book_reviews.into_iter().enumerate() {
            assert_eq!(review, review_answer[idx]);
        }
    }
}
