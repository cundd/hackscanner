use crate::errors::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::sync::RwLock;

type RegexCacheMap = HashMap<String, Regex>;
lazy_static! {
    static ref REGEX_CACHE: RwLock<RegexCacheMap> = RwLock::new(HashMap::new());
}

pub struct RegexCache;

impl RegexCache {
    pub fn build(pattern: &str) -> Result<Regex> {
        if let Some(regex) = RegexCache::get(pattern) {
            return Ok(regex);
        }

        let regex = Regex::new(&format!("(?i){}", pattern))?;
        RegexCache::set(pattern, regex.clone());

        Ok(regex)
    }

    fn get(pattern: &str) -> Option<Regex> {
        match REGEX_CACHE.read() {
            Ok(read_handle) => (*read_handle).get(pattern).cloned(),
            Err(_) => None,
        }
    }

    fn set(pattern: &str, regex: Regex) {
        if let Ok(mut write_handle) = REGEX_CACHE.try_write() {
            (*write_handle).insert(pattern.to_owned(), regex);
        }
    }
}
