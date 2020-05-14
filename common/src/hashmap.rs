use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap as StdHashMap;
use std::hash::BuildHasherDefault;

/// This hashmap does not have random state.
/// Used to fix the order of items in cbor maps.
pub type HashMap<K, V> = StdHashMap<K, V, BuildHasherDefault<DefaultHasher>>;
