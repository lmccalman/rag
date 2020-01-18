use anyhow::{Result, anyhow};
use std::hash::Hash;
use std::collections::HashMap;

pub fn get_result<'a, K: Eq + Hash, V>(id: &'a K, 
                                       map: &'a HashMap<K, V>) -> Result<&'a V> {
    let obj = (*map).get(id).ok_or(anyhow!("Key not found in hashmap"))?;
    return Ok(obj);
}

pub fn get_result_mut<'a, K: Eq + Hash, V>(id: &'a K, 
                                           map: &'a mut HashMap<K, V>) -> Result<&'a mut V> {
    let obj = map.get_mut(id).ok_or(anyhow!("Key not found in hashmap"))?;
    return Ok(obj);
}

