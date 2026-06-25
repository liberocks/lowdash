use lowdash::Entry;
use std::any::Any;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Person {
    pub id: usize,
    pub name: String,
    pub age: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimedRecord {
    pub id: usize,
    pub name: String,
    pub timestamp: SystemTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopyTimedRecord {
    pub id: usize,
    pub timestamp: SystemTime,
}

impl Default for TimedRecord {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            timestamp: UNIX_EPOCH,
        }
    }
}

impl Default for CopyTimedRecord {
    fn default() -> Self {
        Self {
            id: 0,
            timestamp: UNIX_EPOCH,
        }
    }
}

pub fn int_vec(len: usize) -> Vec<i32> {
    (0..len as i32).map(|i| (i % 97) - 48).collect()
}

pub fn duplicate_int_vec(len: usize) -> Vec<i32> {
    (0..len as i32).map(|i| i % 32).collect()
}

pub fn defaulty_int_vec(len: usize) -> Vec<i32> {
    (0..len as i32)
        .map(|i| if i % 3 == 0 { 0 } else { (i % 41) + 1 })
        .collect()
}

pub fn float_vec(len: usize) -> Vec<f64> {
    (0..len).map(|i| (((i % 97) as f64) - 48.0) / 3.0).collect()
}

pub fn people(len: usize) -> Vec<Person> {
    (0..len)
        .map(|i| Person {
            id: i,
            name: format!("person-{}", i % 64),
            age: 20 + (i % 50) as u32,
        })
        .collect()
}

pub fn timed_records(len: usize) -> Vec<TimedRecord> {
    (0..len)
        .map(|i| TimedRecord {
            id: i,
            name: format!("record-{}", i),
            timestamp: UNIX_EPOCH + Duration::from_secs((i * 17) as u64),
        })
        .collect()
}

pub fn timed_records_descending(len: usize) -> Vec<TimedRecord> {
    (0..len)
        .map(|i| TimedRecord {
            id: i,
            name: format!("record-{}", i),
            timestamp: UNIX_EPOCH + Duration::from_secs(((len - i) * 17) as u64),
        })
        .collect()
}

pub fn timed_records_equal(len: usize) -> Vec<TimedRecord> {
    (0..len)
        .map(|i| TimedRecord {
            id: i,
            name: format!("record-{}", i),
            timestamp: UNIX_EPOCH + Duration::from_secs(17),
        })
        .collect()
}

pub fn timed_records_shuffled(len: usize) -> Vec<TimedRecord> {
    let mut records = timed_records(len);
    for i in (1..records.len()).rev() {
        let j = (i * 31 + 7) % (i + 1);
        records.swap(i, j);
    }
    records
}

pub fn copy_timed_records(len: usize) -> Vec<CopyTimedRecord> {
    (0..len)
        .map(|i| CopyTimedRecord {
            id: i,
            timestamp: UNIX_EPOCH + Duration::from_secs((i * 17) as u64),
        })
        .collect()
}

pub fn copy_timed_records_descending(len: usize) -> Vec<CopyTimedRecord> {
    (0..len)
        .map(|i| CopyTimedRecord {
            id: i,
            timestamp: UNIX_EPOCH + Duration::from_secs(((len - i) * 17) as u64),
        })
        .collect()
}

pub fn copy_timed_records_equal(len: usize) -> Vec<CopyTimedRecord> {
    (0..len)
        .map(|i| CopyTimedRecord {
            id: i,
            timestamp: UNIX_EPOCH + Duration::from_secs(17),
        })
        .collect()
}

pub fn copy_timed_records_shuffled(len: usize) -> Vec<CopyTimedRecord> {
    let mut records = copy_timed_records(len);
    for i in (1..records.len()).rev() {
        let j = (i * 31 + 7) % (i + 1);
        records.swap(i, j);
    }
    records
}

pub fn time_vec(len: usize) -> Vec<SystemTime> {
    (0..len)
        .map(|i| UNIX_EPOCH + Duration::from_secs((i * 11) as u64))
        .collect()
}

pub fn numeric_map(len: usize) -> HashMap<String, i32> {
    (0..len)
        .map(|i| (format!("key-{}", i), ((i % 97) as i32) - 48))
        .collect()
}

pub fn numeric_maps(map_count: usize, items_per_map: usize) -> Vec<HashMap<String, i32>> {
    (0..map_count)
        .map(|map_index| {
            (0..items_per_map)
                .map(|item_index| {
                    (
                        format!("key-{}-{}", map_index, item_index),
                        ((map_index + item_index) % 64) as i32,
                    )
                })
                .collect()
        })
        .collect()
}

pub fn map_refs<'a>(maps: &'a [HashMap<String, i32>]) -> Vec<&'a HashMap<String, i32>> {
    maps.iter().collect()
}

pub fn entry_vec(len: usize) -> Vec<Entry<String, i32>> {
    (0..len)
        .map(|i| Entry {
            key: format!("entry-{}", i),
            value: ((i % 97) as i32) - 48,
        })
        .collect()
}

pub fn nested_vecs(outer: usize, inner: usize) -> Vec<Vec<i32>> {
    (0..outer)
        .map(|row| {
            (0..inner)
                .map(|col| (row * inner + col) as i32)
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn ragged_vecs() -> Vec<Vec<i32>> {
    vec![
        vec![1, 2, 3, 4],
        vec![5, 6],
        vec![7, 8, 9],
        vec![10],
        vec![11, 12, 13, 14, 15],
    ]
}

pub fn boxed_float_any(len: usize) -> Vec<Box<dyn Any>> {
    (0..len)
        .map(|i| Box::new(i as f64 / 3.0) as Box<dyn Any>)
        .collect()
}

pub fn mixed_identifier() -> &'static str {
    "HTTPResponse_v2-userProfile 99Bottles_of_Soda"
}

pub fn long_sentence() -> &'static str {
    "Alpha beta-gamma_delta HTTPResponse42 keeps rolling across a long sentence for benchmarking text utilities"
}

pub fn substring_input() -> &'static str {
    "Hello\0Rustaceans_and-Benchmarkers42"
}

pub fn int_vec_descending(len: usize) -> Vec<i32> {
    (0..len as i32).rev().map(|i| (i % 97) - 48).collect()
}

pub fn int_vec_shuffled(len: usize) -> Vec<i32> {
    let mut v = int_vec(len);
    for i in (1..v.len()).rev() {
        let j = (i * 31 + 7) % (i + 1);
        v.swap(i, j);
    }
    v
}

pub fn people_same_age(len: usize) -> Vec<Person> {
    (0..len)
        .map(|i| Person {
            id: i,
            name: format!("person-{}", i % 64),
            age: 30,
        })
        .collect()
}

pub fn people_descending(len: usize) -> Vec<Person> {
    (0..len)
        .map(|i| Person {
            id: i,
            name: format!("person-{}", (len - 1 - i) % 64),
            age: 20 + ((len - 1 - i) % 50) as u32,
        })
        .collect()
}

pub fn people_shuffled(len: usize) -> Vec<Person> {
    let mut v = people(len);
    for i in (1..v.len()).rev() {
        let j = (i * 31 + 7) % (i + 1);
        v.swap(i, j);
    }
    v
}

pub fn short_string() -> &'static str {
    "hello world"
}

pub fn time_vec_descending(len: usize) -> Vec<std::time::SystemTime> {
    (0..len)
        .map(|i| std::time::UNIX_EPOCH + std::time::Duration::from_secs(((len - i) * 11) as u64))
        .collect()
}

pub fn time_vec_equal(len: usize) -> Vec<std::time::SystemTime> {
    vec![std::time::UNIX_EPOCH + std::time::Duration::from_secs(17); len]
}

pub fn time_vec_shuffled(len: usize) -> Vec<std::time::SystemTime> {
    let mut v = time_vec(len);
    for i in (1..v.len()).rev() {
        let j = (i * 31 + 7) % (i + 1);
        v.swap(i, j);
    }
    v
}
