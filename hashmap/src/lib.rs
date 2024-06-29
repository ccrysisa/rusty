// const INITIAL_NBUCKETS: usize = 1;

use std::{
    borrow::Borrow,
    hash::{DefaultHasher, Hash, Hasher},
    mem,
    ops::Index,
};

pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
    count: usize,
}

struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

impl<K, V> Bucket<K, V> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
    entry: &'a mut (K, V),
}

pub struct VacantEntry<'a, K: 'a, V: 'a> {
    key: K,
    map: &'a mut HashMap<K, V>,
    bucket: usize,
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V {
        self.map.count += 1;

        self.map.buckets[self.bucket].items.push((self.key, value));
        self.map.buckets[self.bucket]
            .items
            .last_mut()
            .map(|&mut (_, ref mut value)| value)
            .unwrap()
    }
}

pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + Eq,
{
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => &mut e.entry.1,
            Entry::Vacant(e) => e.insert(value),
        }
    }

    pub fn or_insert_with<F>(self, maker: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(e) => &mut e.entry.1,
            Entry::Vacant(e) => e.insert(maker()),
        }
    }

    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert(V::default())
    }

    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        if let Entry::Occupied(OccupiedEntry {
            entry: &mut (_, ref mut value),
        }) = self
        {
            f(value)
        }
        self
    }
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            count: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        if self.buckets.is_empty() || self.count >= 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self
            .bucket(&key)
            .expect("buckets.is_empty() is handled above!");
        match self.buckets[bucket]
            .items
            .iter()
            .position(|&(ref ekey, _)| ekey == &key)
        {
            Some(index) => Entry::Occupied(OccupiedEntry {
                entry: &mut self.buckets[bucket].items[index],
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                map: self,
                bucket,
            }),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.count >= 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self.bucket(&key)?;
        let bucket = &mut self.buckets[bucket];

        if let Some(&mut (_, ref mut eval)) = bucket
            .items
            .iter_mut()
            .find(|&&mut (ref ekey, _)| ekey == &key)
        {
            Some(mem::replace(eval, value))
        } else {
            bucket.items.push((key, value));
            self.count += 1;
            None
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(key)?;
        self.buckets[bucket]
            .items
            .iter()
            .find(|&&(ref ekey, _)| ekey.borrow() == key)
            .map(|&(_, ref value)| value)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.remove_entry(key).map(|(_, value)| value)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(key)?;
        let bucket = &mut self.buckets[bucket];
        let i = bucket
            .items
            .iter()
            .position(|&(ref ekey, _)| ekey.borrow() == key)?;
        self.count -= 1;
        Some(bucket.items.swap_remove(i))
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn bucket<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.buckets.is_empty() {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        Some((hasher.finish() % self.buckets.len() as u64) as usize)
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => 1, /* INITIAL_NBUCKETS */
            n => 2 * n,
        };

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Bucket::new()));
        for (key, value) in self
            .buckets
            .iter_mut()
            .flat_map(|bucket| bucket.items.drain(..))
        {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % target_size as u64) as usize;
            let bucket = &mut new_buckets[bucket];

            bucket.items.push((key, value));
        }

        self.buckets = new_buckets;
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.items.get(self.at) {
                    Some(&(ref key, ref value)) => {
                        self.at += 1;
                        break Some((key, value));
                    }
                    None => {
                        self.bucket += 1;
                        self.at = 0;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket: 0,
            at: 0,
        }
    }
}

// pub struct IterMut<'a, K: 'a, V: 'a> {
//     map: &'a mut HashMap<K, V>,
//     bucket: usize,
//     at: usize,
// }

// impl<'a, K, V> Iterator for IterMut<'a, K, V> {
//     type Item = (&'a K, &'a mut V);

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             match self.map.buckets.get(self.bucket) {
//                 Some(bucket) => match bucket.items.get(self.at) {
//                     Some(&(ref key, _)) => {
//                         break Some((key, &mut self.map.buckets[self.bucket].items[self.at].1));
//                     }

//                     None => {
//                         self.bucket += 1;
//                         self.at = 0;
//                         continue;
//                     }
//                 },
//                 None => break None,
//             }
//         }
//     }
// }

// impl<'a, K, V> IntoIterator for &'a mut HashMap<K, V> {
//     type Item = (&'a K, &'a mut V);
//     type IntoIter = IterMut<'a, K, V>;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

impl<K, V> HashMap<K, V> {
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.into_iter()
    }

    // pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
    //     self.into_iter()
    // }
}

pub struct IntoIter<K, V> {
    map: HashMap<K, V>,
    bucket: usize,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get_mut(self.bucket) {
                Some(bucket) => match bucket.items.pop() {
                    Some(e) => break Some(e),
                    None => {
                        self.bucket += 1;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            map: self,
            bucket: 0,
        }
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = Self::new();
        for (key, value) in iter {
            map.insert(key, value);
        }
        map
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn from(value: [(K, V); N]) -> Self {
        Self::from_iter(value)
    }
}

impl<K, V, Q> Index<&Q> for HashMap<K, V>
where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        map.insert("foo", 0);
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"foo"), Some(&0));
        assert!(map.contains_key(&"foo"));
        assert_eq!(map.remove(&"foo"), Some(0));
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.get(&"foo"), None);
        assert!(!map.contains_key(&"foo"));
    }

    #[test]
    fn remove() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.remove(&1), None);

        let mut map = HashMap::new();
        map.insert(1, "a");
        assert_eq!(map.remove_entry(&1), Some((1, "a")));
        assert_eq!(map.remove(&1), None);
    }

    #[test]
    fn iter() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        map.insert("bar", 43);
        map.insert("baz", 142);
        map.insert("quox", 7);

        for (&k, &v) in &map {
            match k {
                "foo" => assert_eq!(v, 42),
                "bar" => assert_eq!(v, 43),
                "baz" => assert_eq!(v, 142),
                "quox" => assert_eq!(v, 7),
                _ => unreachable!(),
            }
        }
        assert_eq!(map.iter().count(), 4);

        let mut items = 0;
        for (k, v) in map {
            match k {
                "foo" => assert_eq!(v, 42),
                "bar" => assert_eq!(v, 43),
                "baz" => assert_eq!(v, 142),
                "quox" => assert_eq!(v, 7),
                _ => unreachable!(),
            }
            items += 1;
        }
        assert_eq!(items, 4);

        // let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
        // // Update all values
        // for (_, val) in map.iter_mut() {
        //     *val *= 2;
        // }
        // for (key, val) in &map {
        //     println!("key: {key} val: {val}");
        // }
        // assert_eq!(map.iter().count(), 4);
    }

    #[test]
    fn empty_hashmap() {
        let mut map: HashMap<String, &str> = HashMap::new();
        assert_eq!(map.contains_key("key"), false);
        assert_eq!(map.get("key"), None);
        assert_eq!(map.remove("key"), None);
    }

    #[test]
    fn and_modify() {
        let mut map: HashMap<&str, u32> = HashMap::new();

        map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
        assert_eq!(map["poneyland"], 42);

        map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
        assert_eq!(map["poneyland"], 43);
    }
}
