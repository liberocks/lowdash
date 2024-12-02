# Lowdash

![Build Status](https://github.com/liberocks/lowdash/actions/workflows/build.yml/badge.svg)
[![Coverage](https://coveralls.io/repos/github/liberocks/lowdash/badge.svg?branch=master)](https://coveralls.io/github/liberocks/lowdash?branch=master)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/6d7316eb78084cc6bbe1152ee7ac51f7)](https://app.codacy.com/gh/liberocks/lowdash/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Contributors](https://img.shields.io/github/contributors/liberocks/lowdash)](https://github.com/liberocks/lowdash/graphs/contributors)
[![License](https://img.shields.io/github/license/liberocks/lowdash)](./LICENSE)

**liberocks/lowdash** is a Lodash inspired utility library to manipulate array and object

## ⚠️ Disclaimer
This project is still in heavy development and not ready for production use.

## 🚀 Installation
```bash
cargo add lowdash
```

This library has no dependencies outside the Rust standard library.

## 📚 Documentation
You can find the generated documentation [here](https://docs.rs/lowdash)

Utility functions for array:
- [associate](#associate)
- [chunk](#chunk)
- [compact](#compact)
- [count](#count)
- [count_by](#count_by)
- [count_values](#count_values)
- [count_values_by](#count_values_by)
- [drop](#drop)
- [drop_right](#drop_right)
- [drop_right_while](#drop_right_while)
- [drop_while](#drop_while)
- [earliest](#earliest)
- [earliest_by](#earliest_by)
- [fill](#fill)
- [filter](#filter)
- [filter_map](#filter_map)
- [filter_reject](#filter_reject)
- [find](#find)
- [find_duplicates](#find_duplicates)
- [find_duplicates_by](#find_duplicates_by)
- [find_index_of](#find_index_of)
- [find_key](#find_key)
- [find_key_by](#find_key_by)
- [find_last_index_of](#find_last_index_of)
- [find_or_else](#find_or_else)
- [find_uniques](#find_uniques)
- [find_uniques_by](#find_uniques_by)
- [first](#first)
- [first_or](#first_or)
- [first_or_empty](#first_or_empty)
- [flat_map](#flat_map)
- [flatten](#flatten)
- [foreach](#foreach)
- [foreach_while](#foreach_while)
- [group_by](#group_by)
- [index_of](#index_of)
- [interleave](#interleave)
- [is_sorted](#is_sorted)
- [is_sorted_by_key](#is_sorted_by_key)
- [key_by](#key_by)
- [last](#last)
- [last_index_of](#last_index_of)
- [last_or](#last_or)
- [last_or_empty](#last_or_empty)
- [latest](#latest)
- [latest_by](#latest_by)
- [map](#map)
- [max](#max)
- [max_by](#max_by)
- [min](#min)
- [min_by](#min_by)
- [nth](#nth)
- [partition_by](#partition_by)
- [reduce](#reduce)
- [reduce_right](#reduce_right)
- [reject](#reject)
- [reject_map](#reject_map)
- [repeat](#repeat)
- [repeat_by](#repeat_by)
- [replace](#replace)
- [replace_all](#replace_all)
- [reverse](#reverse)
- [sample](#sample)
- [samples](#samples)
- [shuffle](#shuffle)
- [slice](#slice)
- [slice_to_map](#slice_to_map)
- [splice](#splice)
- [subset](#subset)
- [times](#times)
- [uniq](#uniq)
- [uniq_by](#uniq_by)
- [drop_by_index](#drop_by_index)

Utility functions for string manipulation:
- [camel_case](#camel_case)
- [capitalize](#capitalize)
- [char_length](#char_length)
- [chunk_string](#chunk_string)
- [ellipsis](#ellipsis)
- [kebab_case](#kebab_case)
- [pascal_case](#pascal_case)
- [random_string](#random_string)
- [snake_case](#snake_case)
- [substring](#substring)
- [words](#words)

Utility functions for math:
- [nearest_power_of_two](#nearest_power_of_two)

### camel_case
Converts a string to camelCase.

```rust
use lowdash::camel_case;

assert_eq!(camel_case("hello world"), "helloWorld");
assert_eq!(camel_case("foo-bar"), "fooBar");
assert_eq!(camel_case("lorem_ipsum"), "loremIpsum");
assert_eq!(camel_case("FooBarBazHello"), "fooBarBazHello");
```

### capitalize
Capitalizes the first letter of the input string and converts the rest to lowercase.

```rust
use lowdash::capitalize;

assert_eq!(capitalize("hello"), "Hello");
assert_eq!(capitalize("WORLD"), "World");
assert_eq!(capitalize("rUsT"), "Rust");
```
### char_length
Returns the length of a string in Unicode characters.

```rust
use lowdash::char_length;

assert_eq!(char_length("hello"), 5);
assert_eq!(char_length("🌍world"), 6);
assert_eq!(char_length("こんにちは"), 5);
```

### chunk_string
Splits a string into chunks of specified size.

```rust
use lowdash::chunk_string;

let result = chunk_string("hello", 2);
assert_eq!(result, vec!["he", "ll", "o"]);
```

### earliest_by
Find the earliest item in a collection based on a custom iteratee function.

```rust
use std::time::{SystemTime, Duration};
use lowdash::earliest_by;

let t1 = SystemTime::UNIX_EPOCH;
let t2 = t1 + Duration::new(60, 0);
let t3 = t1 + Duration::new(120, 0);
let times = vec![t2, t1, t3];
let earliest_time = earliest_by(&times, |&t| t);
assert_eq!(earliest_time, Some(t1));

#[derive(Debug, PartialEq, Clone)]
struct Event {
    name: String,
    timestamp: SystemTime,
}

let events = vec![
    Event {
        name: "Event1".to_string(),
        timestamp: t2,
    },
    Event {
        name: "Event2".to_string(),
        timestamp: t1,
    },
    Event {
        name: "Event3".to_string(),
        timestamp: t3,
    },
];

let earliest_event = earliest_by(&events, |e| e.timestamp);
assert_eq!(
    earliest_event,
    Some(Event {
        name: "Event2".to_string(),
        timestamp: t1,
    })
);
```

### earliest
Find the earliest time in a collection.

```rust
use std::time::{SystemTime, Duration};
use lowdash::earliest;

let t1 = SystemTime::UNIX_EPOCH;
let t2 = t1 + Duration::new(60, 0);
let t3 = t1 + Duration::new(120, 0);
let times = vec![t2, t1, t3];
let earliest_time = earliest(&times);
assert_eq!(earliest_time, Some(t1));
```

### ellipsis
Truncates a string and appends an ellipsis (`"..."`) if it exceeds the specified length.

```rust
use lowdash::ellipsis;

let result = ellipsis("Hello, World!", 10);
assert_eq!(result, "Hello, ...");

let result = ellipsis("Short", 10);
assert_eq!(result, "Short");

let result = ellipsis("ExactLength", 11);
assert_eq!(result, "ExactLength");

let result = ellipsis("  Trimmed  ", 6);
assert_eq!(result, "Tri...");

let result = ellipsis("Hi", 2);
assert_eq!(result, "Hi");
```

### find_duplicates_by
Find all duplicate elements in a collection based on a key generated by the iteratee function.

```rust
use lowdash::find_duplicates_by;
let numbers = vec![1, 2, 3, 4];
let result = find_duplicates_by(&numbers, |x| x % 2);
assert_eq!(result, vec![3, 4]); // Second occurrences of duplicated keys
```

```rust
use lowdash::find_duplicates_by;

#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 25 },
];

let result = find_duplicates_by(&people, |p| p.age);
assert_eq!(result, vec![
    Person { name: "Carol".to_string(), age: 25 },
]);
```

### find_duplicates
Find all duplicate elements in a collection (elements that appear more than once).

```rust
use lowdash::find_duplicates;
let numbers = vec![1, 2, 2, 3, 3, 4];
let result = find_duplicates(&numbers);
assert_eq!(result, vec![2, 3]);
```

```rust
use lowdash::find_duplicates;
let words = vec!["apple", "banana", "apple", "cherry", "banana"];
let result = find_duplicates(&words);
assert_eq!(result, vec!["apple", "banana"]);
```

```rust
use lowdash::find_duplicates;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
];

let result = find_duplicates(&people);
assert_eq!(result, vec![Person { name: "Alice".to_string(), age: 25 }]);
```

### find_index_of
Find the first item in a collection that satisfies a predicate and return its index.

```rust
use lowdash::find_index_of;
let numbers = vec![1, 2, 3, 4, 5];
let predicate = |x: &i32| *x == 3;
let result = find_index_of(&numbers, predicate);
assert_eq!(result, Some((&3, 2)));
```

```rust
use lowdash::find_index_of;
let numbers = vec![10, 20, 30, 40];
let result = find_index_of(&numbers, |x| *x > 25);
assert_eq!(result, Some((&30, 2)));
```

```rust
use lowdash::find_index_of;

#[derive(Debug, PartialEq)]
struct Person {
   name: String,
   age: u32,
}

let people = vec![
   Person { name: "Alice".to_string(), age: 25 },
   Person { name: "Bob".to_string(), age: 30 },
   Person { name: "Carol".to_string(), age: 35 },
];

let result = find_index_of(&people, |p| p.age > 30);
assert_eq!(result, Some((&Person { name: "Carol".to_string(), age: 35 }, 2)));
```

### find_key_by
Find the key in a map that satisfies a predicate based on both key and value.

```rust
use lowdash::find_key_by;
let mut map = std::collections::HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);
let result = find_key_by(&map, |k, v| *k == "b" && *v == 2);
assert_eq!(result, Some(&"b"));
```

```rust
use lowdash::find_key_by;
let mut map = std::collections::HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);
let result = find_key_by(&map, |_, v| *v > 2);
assert_eq!(result, Some(&"c"));
```

### find_key
Find the key in a map that corresponds to a given value.

```rust
use lowdash::find_key;
let mut map = std::collections::HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);
let result = find_key(&map, 2);
assert_eq!(result, Some(&"b"));
```

```rust
use lowdash::find_key;
let mut map = std::collections::HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);
let result = find_key(&map, 4);
assert_eq!(result, None);
```

```rust
use lowdash::find_key;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Person {
   name: String,
   age: u32,
}

let mut map = std::collections::HashMap::new();
map.insert(Person { name: "Alice".to_string(), age: 25 }, "Engineer");
map.insert(Person { name: "Bob".to_string(), age: 30 }, "Manager");
map.insert(Person { name: "Carol".to_string(), age: 35 }, "Director");

let result = find_key(&map, "Manager");
assert_eq!(result, Some(&Person { name: "Bob".to_string(), age: 30 }));
```

### find_last_index_of
Find the last item in a collection that satisfies a predicate and return its index.

```rust
use lowdash::find_last_index_of;
let numbers = vec![1, 2, 3, 4, 5, 3];
let predicate = |x: &i32| *x == 3;
let result = find_last_index_of(&numbers, predicate);
assert_eq!(result, Some((&3, 5)));
```

```rust
use lowdash::find_last_index_of;
let numbers = vec![10, 20, 30, 40, 30];
let result = find_last_index_of(&numbers, |x| *x > 25);
assert_eq!(result, Some((&30, 4)));
```

```rust
use lowdash::find_last_index_of;

#[derive(Debug, PartialEq)]
struct Person {
   name: String,
   age: u32,
}

let people = vec![
   Person { name: "Alice".to_string(), age: 25 },
   Person { name: "Bob".to_string(), age: 30 },
   Person { name: "Carol".to_string(), age: 35 },
   Person { name: "Dave".to_string(), age: 35 },
];

let result = find_last_index_of(&people, |p| p.age > 30);
assert_eq!(result, Some((&Person { name: "Dave".to_string(), age: 35 }, 3)));
```

### find_or_else
Find the first item in a collection that satisfies a predicate.

```rust
use lowdash::find_or_else;
let numbers = vec![1, 2, 3, 4, 5];
let predicate = |x: &i32| *x == 3;
let result = find_or_else(&numbers, &0, predicate);
assert_eq!(result, &3);
```

```rust
use lowdash::find_or_else;
let numbers = vec![10, 20, 30, 40];
let result = find_or_else(&numbers, &0, |x| *x > 50);
assert_eq!(result, &0);
```

```rust
use lowdash::find_or_else;

#[derive(Debug, PartialEq)]
struct Person {
   name: String,
   age: u32,
}

let people = vec![
   Person { name: "Alice".to_string(), age: 25 },
   Person { name: "Bob".to_string(), age: 30 },
   Person { name: "Carol".to_string(), age: 35 },
];

let fallback = Person { name: "Unknown".to_string(), age: 0 };
let result = find_or_else(&people, &fallback, |p| p.age > 30);
assert_eq!(result, &Person { name: "Carol".to_string(), age: 35 });
```

### find_uniques
Find all unique elements in a collection (elements that appear exactly once).

```rust
use lowdash::find_uniques;
let numbers = vec![1, 2, 2, 3, 3, 4];
let result = find_uniques(&numbers);
assert_eq!(result, vec![1, 4]);
```

```rust
use lowdash::find_uniques;
let words = vec!["apple", "banana", "apple", "cherry"];
let result = find_uniques(&words);
assert_eq!(result, vec!["banana", "cherry"]);
```

```rust
use lowdash::find_uniques;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
];

let result = find_uniques(&people);
assert_eq!(result, vec![Person { name: "Bob".to_string(), age: 30 }]);
```

### find_uniques_by
Find all unique elements in a collection based on a key generated by the iteratee function.

```rust
use lowdash::find_uniques_by;
let numbers = vec![1, 2, 3, 4];
let result = find_uniques_by(&numbers, |x| x % 2);  // Group by even/odd
assert_eq!(result, vec![]);  // No unique remainders
```

```rust
use lowdash::find_uniques_by;

#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 25 },
];

let result = find_uniques_by(&people, |p| p.age);
assert_eq!(result, vec![Person { name: "Bob".to_string(), age: 30 }]);
```

### find
Find the first item in a collection that satisfies a predicate.

```rust
use lowdash::find;
let numbers = vec![1, 2, 3, 4, 5];
let predicate = |x: &i32| *x == 3;
let result = find(&numbers, predicate);
assert_eq!(result, Some(&3));
```

```rust
use lowdash::find;
let numbers = vec![10, 20, 30, 40];
let result = find(&numbers, |x| *x > 25);
assert_eq!(result, Some(&30));
```

```rust
use lowdash::find;

#[derive(Debug, PartialEq)]
struct Person {
   name: String,
   age: u32,
}

let people = vec![
   Person { name: "Alice".to_string(), age: 25 },
   Person { name: "Bob".to_string(), age: 30 },
   Person { name: "Carol".to_string(), age: 35 },
];

let result = find(&people, |p| p.age > 30);
assert_eq!(result, Some(&Person { name: "Carol".to_string(), age: 35 }));
```

### first_or_empty
Returns the first item from the collection.

```rust
use lowdash::first_or_empty;

let numbers = vec![1, 2, 3];
let first_num = first_or_empty(&numbers);
assert_eq!(first_num, 1);

let empty: Vec<i32> = vec![];
let first_num = first_or_empty(&empty);
assert_eq!(first_num, 0); // i32::default() is 0
```

```rust
use lowdash::first_or_empty;

#[derive(Debug, PartialEq, Clone, Default)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
];

let first_person = first_or_empty(&people);
assert_eq!(first_person, Person { name: "Alice".to_string(), age: 25 });

let empty_people: Vec<Person> = vec![];
let first_person = first_or_empty(&empty_people);
assert_eq!(first_person, Person::default());
```

### first_or
Returns the first item from the collection.

```rust
use lowdash::first_or;

let numbers = vec![1, 2, 3];
let first_num = first_or(&numbers, 10);
assert_eq!(first_num, 1);

let empty: Vec<i32> = vec![];
let first_num = first_or(&empty, 10);
assert_eq!(first_num, 10);
```

```rust
use lowdash::first_or;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
];

let first_person = first_or(&people, Person { name: "Default".to_string(), age: 0 });
assert_eq!(first_person, Person { name: "Alice".to_string(), age: 25 });

let empty_people: Vec<Person> = vec![];
let first_person = first_or(&empty_people, Person { name: "Default".to_string(), age: 0 });
assert_eq!(first_person, Person { name: "Default".to_string(), age: 0 });
```

### first
Returns the first item from the collection.

```rust
use lowdash::first;

let numbers = vec![1, 2, 3];
let (first_num, exists) = first(&numbers);
assert_eq!(first_num, 1);
assert!(exists);

let empty: Vec<i32> = vec![];
let (first_num, exists) = first(&empty);
assert_eq!(first_num, 0); // i32::default() is 0
assert!(!exists);
```

### index_of
Finds the position of the first occurrence of an element in a collection.

```rust
use lowdash::index_of;
let collection = vec![1, 2, 3, 4, 5];
let index = index_of(&collection, 3);
assert_eq!(index, 2);
```

```rust
use lowdash::index_of;
let collection = vec!["apple", "banana", "cherry"];
let index = index_of(&collection, "banana");
assert_eq!(index, 1);
```

```rust
use lowdash::index_of;
let collection = vec![1, 2, 3, 4, 5];
let index = index_of(&collection, 6);
assert_eq!(index, -1);
```

### kebab_case
Converts a string to kebab-case.

```rust
use lowdash::kebab_case;

assert_eq!(kebab_case("hello world"), "hello-world");
assert_eq!(kebab_case("foo-bar"), "foo-bar");
assert_eq!(kebab_case("lorem_ipsum"), "lorem-ipsum");
assert_eq!(kebab_case("FooBarBazHello"), "foo-bar-baz-hello");
assert_eq!(kebab_case("helloWorld"), "hello-world");
```

### last_index_of
Finds the position of the last occurrence of an element in a collection.

```rust
use lowdash::last_index_of;
let collection = vec![1, 2, 3, 2, 1];
let index = last_index_of(&collection, 2);
assert_eq!(index, 3);
```

```rust
use lowdash::last_index_of;
let collection = vec!["apple", "banana", "cherry", "banana"];
let index = last_index_of(&collection, "banana");
assert_eq!(index, 3);
```

```rust
use lowdash::last_index_of;
let collection = vec![1, 2, 3, 4, 5];
let index = last_index_of(&collection, 6);
assert_eq!(index, -1);
```

### last_or_empty
Returns the last item from the collection.

```rust
use lowdash::last_or_empty;

let numbers = vec![1, 2, 3];
let last_num = last_or_empty(&numbers);
assert_eq!(last_num, 3);

let empty: Vec<i32> = vec![];
let last_num = last_or_empty(&empty);
assert_eq!(last_num, 0); // i32::default() is 0
```

```rust
use lowdash::last_or_empty;

#[derive(Debug, PartialEq, Clone, Default)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
];

let last_person = last_or_empty(&people);
assert_eq!(last_person, Person { name: "Bob".to_string(), age: 30 });

let empty_people: Vec<Person> = vec![];
let last_person = last_or_empty(&empty_people);
assert_eq!(last_person, Person::default());
```

### last_or
Returns the last item from the collection.

```rust
use lowdash::last_or;

let numbers = vec![1, 2, 3];
let last_num = last_or(&numbers, 10);
assert_eq!(last_num, 3);

let empty: Vec<i32> = vec![];
let last_num = last_or(&empty, 10);
assert_eq!(last_num, 10);
```

### last
Returns the last item from the collection.

```rust
use lowdash::last;

let numbers = vec![1, 2, 3];
let (last_num, exists) = last(&numbers);
assert_eq!(last_num, 3);
assert!(exists);

let empty: Vec<i32> = vec![];
let (last_num, exists) = last(&empty);
assert_eq!(last_num, 0); // i32::default() is 0
assert!(!exists);
```

```rust
use lowdash::last;

#[derive(Debug, PartialEq, Clone, Default)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
];

let (last_person, exists) = last(&people);
assert_eq!(last_person, Person { name: "Bob".to_string(), age: 30 });
assert!(exists);

let empty_people: Vec<Person> = vec![];
let (last_person, exists) = last(&empty_people);
assert_eq!(last_person, Person::default());
assert!(!exists);
```

### latest_by
Returns the item from the collection for which the iteratee returns the latest `SystemTime`.

```rust
use std::time::{SystemTime, Duration};
use lowdash::latest_by;

#[derive(Debug, PartialEq, Clone)]
struct Record {
    id: u32,
    timestamp: SystemTime,
}

impl Default for Record {
    fn default() -> Self {
        Record {
            id: 0,
            timestamp: SystemTime::UNIX_EPOCH,
        }
    }
}

let records = vec![
    Record {
        id: 1,
        timestamp: SystemTime::UNIX_EPOCH + Duration::new(100, 0),
    },
    Record {
        id: 2,
        timestamp: SystemTime::UNIX_EPOCH + Duration::new(200, 0),
    },
    Record {
        id: 3,
        timestamp: SystemTime::UNIX_EPOCH + Duration::new(150, 0),
    },
];

let latest_record = latest_by(&records, |r| r.timestamp);
assert_eq!(latest_record.id, 2);
```

### latest
Returns the latest `SystemTime` from the provided arguments.

```rust
use std::time::{SystemTime, Duration};
use lowdash::latest;

let now = SystemTime::now();
let later = now + Duration::new(10, 0);
let latest_time = latest(&[now, later]);
assert_eq!(latest_time, later);
```

### max_by
Find the maximum element in a collection based on a custom comparison function.

```rust
use lowdash::max_by;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Person {
    age: u32,
    name: String,
}

let people = vec![
    Person { age: 25, name: "Alice".to_string() },
    Person { age: 30, name: "Bob".to_string() },
    Person { age: 20, name: "Carol".to_string() },
];

let result = max_by(&people, |a, b| a.age > b.age);
assert_eq!(
    result,
    Some(Person { age: 30, name: "Bob".to_string() })
);
```

### max
Find the maximum element in a collection.

```rust
use lowdash::max;
let numbers = vec![5, 3, 8, 1, 4];
let result = max(&numbers);
assert_eq!(result, Some(8));
```

```rust
use lowdash::max;
let strings = vec!["apple", "banana", "cherry"];
let result = max(&strings);
assert_eq!(result, Some("cherry"));
```

```rust
use lowdash::max;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Cia".to_string(), age: 20 },
];

let result = max(&people);
assert_eq!(
    result,
    Some(Person { name: "Cia".to_string(), age: 20 })
);
```

```rust
use lowdash::max;
let collection = vec![3.14, 2.71, -1.0, 0.0];
let result = max(&collection);
assert_eq!(result, Some(3.14));
```

### min_by
Find the minimum element in a collection based on a custom comparison function.

```rust
use lowdash::min_by;

let numbers = vec![5, 3, 8, 1, 4];
let min = min_by(&numbers, |a, b| a < b);
assert_eq!(min, Some(1));

let strings = vec!["apple", "banana", "cherry"];
let min = min_by(&strings, |a, b| a.len() < b.len());
assert_eq!(min, Some("apple"));

let empty: Vec<i32> = vec![];
let min = min_by(&empty, |a, b| a < b);
assert_eq!(min, None);
```

### min
Find the minimum element in a collection.

```rust
use lowdash::min;
let numbers = vec![5, 3, 8, 1, 4];
let result = min(&numbers);
assert_eq!(result, Some(1));
```

```rust
use lowdash::min;
let strings = vec!["apple", "banana", "cherry"];
let result = min(&strings);
assert_eq!(result, Some("apple"));
```

```rust
use lowdash::min;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 20 },
];

let result = min(&people);
assert_eq!(
    result,
    Some(Person { name: "Alice".to_string(), age: 25 })
);
```

```rust
use lowdash::min;
let collection = vec![3.14, 2.71, -1.0, 0.0];
let result = min(&collection);
assert_eq!(result, Some(-1.0));
```

### nearest_power_of_two
Calculates the smallest power of two greater than or equal to the given capacity.

```rust
use lowdash::nearest_power_of_two;

assert_eq!(nearest_power_of_two(0), 1);
assert_eq!(nearest_power_of_two(1), 1);
assert_eq!(nearest_power_of_two(5), 8);
assert_eq!(nearest_power_of_two(16), 16);
assert_eq!(nearest_power_of_two(17), 32);
```

### nth
Returns the nth element from the collection.

```rust
use lowdash::nth;

let numbers = vec![1, 2, 3, 4, 5];
let result = nth(&numbers, 2);
assert_eq!(result.unwrap(), &3);

let result = nth(&numbers, -2);
assert_eq!(result.unwrap(), &4);

let result = nth(&numbers, 10);
assert!(result.is_err());
```

### pascal_case
Converts a string to PascalCase.

```rust
use lowdash::pascal_case;

assert_eq!(pascal_case("hello world"), "HelloWorld");
assert_eq!(pascal_case("foo-bar"), "FooBar");
assert_eq!(pascal_case("lorem_ipsum"), "LoremIpsum");
```

### random_string
Generates a random string of a specified size using the provided charset.

```rust
use lowdash::common::ALPHANUMERIC_CHARSET;
use lowdash::random_string;

let charset = ALPHANUMERIC_CHARSET;
let random_str = random_string(10, charset);
assert_eq!(random_str.len(), 10);
for c in random_str.chars() {
    assert!(charset.contains(&c));
}
```

### sample
Returns a pseudo-random element from the collection.

```rust
use lowdash::sample;

let numbers = vec![1, 2, 3, 4, 5];
let result = sample(&numbers);
assert!(numbers.contains(&result));

let empty: Vec<i32> = vec![];
let result = sample(&empty);
assert_eq!(result, 0); // i32::default()
```

### samples
Returns a slice of pseudo-randomly selected elements from the collection.

```rust
use lowdash::samples;

let numbers = vec![1, 2, 3, 4, 5];
let result = samples(&numbers, 3);
assert_eq!(result.len(), 3);
assert!(result.iter().all(|x| numbers.contains(x)));
```

### snake_case
Converts a string to snake_case.

```rust
use lowdash::snake_case;

assert_eq!(snake_case("hello world"), "hello_world");
assert_eq!(snake_case("foo-bar"), "foo_bar");
assert_eq!(snake_case("lorem_ipsum"), "lorem_ipsum");
assert_eq!(snake_case("FooBarBazHello"), "foo_bar_baz_hello");
assert_eq!(snake_case("fooBarBazHello"), "foo_bar_baz_hello");
```

### substring
Extracts a substring from the given string based on the specified offset and length.

```rust
use lowdash::substring;

let s = String::from("Hello, World!");
assert_eq!(substring(&s, 7, 5), "World");

let s = String::from("Hello, World!");
assert_eq!(substring(&s, -6, 5), "World");

let s = String::from("Hello, World!");
assert_eq!(substring(&s, 100, 5), "");

let s = String::from("Hello\x00World!");
assert_eq!(substring(&s, 0, 10), "HelloWorld");
```

### words
Splits a string into words based on casing, digits, and separators.

```rust
use lowdash::words;

let result = words("Int8Value");
assert_eq!(result, vec!["Int", "8", "Value"]);

let result = words("hello_world");
assert_eq!(result, vec!["hello", "world"]);

let result = words("fooBarBazHello");
assert_eq!(result, vec!["foo", "Bar", "Baz", "Hello"]);
```

### reject
Reject items from a collection that satisfy a predicate.

```rust
use lowdash::reject;
let numbers = vec![1, 2, 3, 4, 5];
let result = reject(&numbers, |x, _| *x % 2 == 0);
assert_eq!(result, vec![&1, &3, &5]);
```

### filter
Filter items from a collection that satisfy a predicate.

```rust
use lowdash::filter;
let numbers = vec![1, 2, 3, 4, 5];
let result = filter(&numbers, |x, _| *x % 2 == 0);
assert_eq!(result, vec![&2, &4]);
```

```rust
use lowdash::filter;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

let result = filter(&people, |p, _| p.age > 30);
assert_eq!(result, vec![&people[2]]);
```

### map
Apply a function to each item in a collection, producing a new collection of results.

```rust
use lowdash::map;
let numbers = vec![1, 2, 3, 4, 5];
let result = map(&numbers, |x, _| x * 2);
assert_eq!(result, vec![2, 4, 6, 8, 10]);
```

```rust
use lowdash::map;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

let names: Vec<String> = map(&people, |p, _| p.name.clone());
assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
```

### filter_map
Apply a function to each item in a collection, filtering and transforming items based on a callback.

```rust
use lowdash::filter_map;
let numbers = vec![1, 2, 3, 4, 5];
// Double even numbers
let result = filter_map(&numbers, |x, _| {
    if *x % 2 == 0 {
        (x * 2, true)
    } else {
        (0, false)
    }
});
assert_eq!(result, vec![4, 8]);
```

```rust
use lowdash::filter_map;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

// Extract names of people older than 28
let names: Vec<String> = filter_map(&people, |p, _| {
    if p.age > 28 {
        (p.name.clone(), true)
    } else {
        (String::new(), false)
    }
});
assert_eq!(names, vec!["Bob".to_string(), "Carol".to_string()]);
```

### flat_map
Apply a function to each item in a collection, flattening the results based on a callback.

```rust
use lowdash::flat_map;
let numbers = vec![1, 2, 3];
// For each number, generate a vector containing the number and its double
let result = flat_map(&numbers, |x, _| vec![*x, *x * 2]);
assert_eq!(result, vec![1, 2, 2, 4, 3, 6]);
```

```rust
use lowdash::flat_map;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    hobbies: Vec<String>,
}

let people = vec![
    Person {
        name: "Alice".to_string(),
        hobbies: vec!["Reading".to_string(), "Cycling".to_string()],
    },
    Person {
        name: "Bob".to_string(),
        hobbies: vec!["Cooking".to_string()],
    },
];

// Extract all hobbies from the list of people
let all_hobbies: Vec<String> = flat_map(&people, |person, _| person.hobbies.clone());
assert_eq!(
    all_hobbies,
    vec![
        "Reading".to_string(),
        "Cycling".to_string(),
        "Cooking".to_string()
    ]
);
```

### reduce
Apply a function to each item in a collection, accumulating a single result.

```rust
use lowdash::reduce;
let numbers = vec![1, 2, 3, 4, 5];
// Sum of all numbers
let sum = reduce(&numbers, |acc, x, _| acc + x, 0);
assert_eq!(sum, 15);
```

```rust
use lowdash::reduce;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

// Concatenate all names
let all_names = reduce(&people, |acc, person, _| format!("{} {}", acc, person.name), String::new());
assert_eq!(all_names.trim(), "Alice Bob Carol");
```

### reduce_right
Apply a function to each item in a collection, accumulating a single result from right to left.

```rust
use lowdash::reduce_right;
let numbers = vec![1, 2, 3, 4, 5];
// Sum of all numbers using reduce_right
let sum = reduce_right(&numbers, |acc, x, _| acc + x, 0);
assert_eq!(sum, 15);
```

```rust
use lowdash::reduce_right;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

// Concatenate all names in reverse order
let all_names = reduce_right(&people, |acc, person, _| {
    if acc.is_empty() {
        person.name.clone()
    } else {
        format!("{} {}", acc, person.name)
    }
}, String::new());
assert_eq!(all_names, "Carol Bob Alice");
```

### foreach
Execute a function on each item in a collection.

```rust
use lowdash::foreach;
let numbers = vec![1, 2, 3, 4, 5];
let mut sum = 0;
foreach(&numbers, |x, _| sum += x);
assert_eq!(sum, 15);
```

```rust
use lowdash::foreach;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

let mut names = Vec::new();
foreach(&people, |p, _| names.push(p.name.clone()));
assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
```

### foreach_while
Execute a function on each item in a collection until the iteratee returns `false`.

```rust
use lowdash::foreach_while;
let numbers = vec![1, 2, 3, 4, 5];
let mut sum = 0;
foreach_while(&numbers, |x, _| {
    sum += x;
    true
});
assert_eq!(sum, 15);
```

```rust
use lowdash::foreach_while;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

let mut names = Vec::new();
foreach_while(&people, |p, _| {
    names.push(p.name.clone());
    true
});
assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
```

```rust
use lowdash::foreach_while;

let numbers = vec![10, 20, 30, 40, 50];
let mut collected = Vec::new();
foreach_while(&numbers, |x, index| {
    if *x < 35 {
        collected.push((*x, index));
        true
    } else {
        false
    }
});
assert_eq!(collected, vec![(10, 0), (20, 1), (30, 2)]);
```

### times
Generates a collection by invoking the provided function `iteratee` a specified number of times.

```rust
use lowdash::times;
let result = times(5, |i| i * 2);
assert_eq!(result, vec![0, 2, 4, 6, 8]);
```

```rust
use lowdash::times;
let result = times(3, |i| format!("Item {}", i));
assert_eq!(result, vec!["Item 0", "Item 1", "Item 2"]);
```

### uniq
Remove duplicate elements from a collection, preserving the order of their first occurrence.

```rust
use lowdash::uniq;
let numbers = vec![1, 2, 2, 3, 4, 3, 5];
let unique_numbers = uniq(&numbers);
assert_eq!(unique_numbers, vec![1, 2, 3, 4, 5]);
```

```rust
use lowdash::uniq;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Carol".to_string(), age: 35 },
];

let unique_people = uniq(&people);
assert_eq!(unique_people, vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
]);
```

### uniq_by
Remove duplicate elements from a collection based on a key extracted by a provided function,
preserving the order of their first occurrence.

```rust
use lowdash::uniq_by;
let numbers = vec![1, 2, 2, 3, 4, 3, 5];
let unique_numbers = uniq_by(&numbers, |x| *x);
assert_eq!(unique_numbers, vec![1, 2, 3, 4, 5]);
```

```rust
use lowdash::uniq_by;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Carol".to_string(), age: 35 },
];

let unique_people = uniq_by(&people, |p| p.name.clone());
assert_eq!(unique_people, vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
]);
```

### group_by
Group elements of a collection based on a key extracted by a provided function,
preserving the order of their first occurrence.

```rust
use lowdash::group_by;

let numbers = vec![1, 2, 2, 3, 4, 3, 5];
let grouped = group_by(&numbers, |x| *x % 2 == 0);

assert_eq!(grouped.get(&false), Some(&vec![1, 3, 3, 5]));
assert_eq!(grouped.get(&true), Some(&vec![2, 2, 4]));
```

```rust
use lowdash::group_by;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 25 },
];

let grouped = group_by(&people, |p| p.age);

assert_eq!(grouped.get(&25), Some(&vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Carol".to_string(), age: 25 },
]));
assert_eq!(grouped.get(&30), Some(&vec![
    Person { name: "Bob".to_string(), age: 30 },
]));
```

### chunk
Divide a collection into smaller chunks of a specified size,
preserving the order of elements.

```rust
use lowdash::chunk;

let numbers = vec![1, 2, 3, 4, 5, 6, 7];
let chunks = chunk(&numbers, 3);
assert_eq!(
    chunks,
    vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]
);
```

```rust
use lowdash::chunk;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
    Person { name: "Dave".to_string(), age: 40 },
];

let chunks = chunk(&people, 2);
assert_eq!(
    chunks,
    vec![
        vec![
            Person { name: "Alice".to_string(), age: 25 },
            Person { name: "Bob".to_string(), age: 30 },
        ],
        vec![
            Person { name: "Carol".to_string(), age: 35 },
            Person { name: "Dave".to_string(), age: 40 },
        ],
    ]
);
```

### partition_by
Divide a collection into partitions based on a key extracted by a provided function,
preserving the order of elements and the order of partitions as they first appear.

```rust
use lowdash::partition_by;

let numbers = vec![1, 2, 2, 3, 4, 3, 5];
let partitions = partition_by(&numbers, |x| *x);
assert_eq!(
    partitions,
    vec![vec![1], vec![2, 2], vec![3, 3], vec![4], vec![5]]
);
```

```rust
use lowdash::partition_by;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Carol".to_string(), age: 35 },
];

let partitions = partition_by(&people, |p| p.age);
assert_eq!(
    partitions,
    vec![
        vec![
            Person { name: "Alice".to_string(), age: 25 },
            Person { name: "Alice".to_string(), age: 25 },
        ],
        vec![
            Person { name: "Bob".to_string(), age: 30 },
        ],
        vec![
            Person { name: "Carol".to_string(), age: 35 },
        ],
    ]
);
```

### flatten
Flatten a collection of slices into a single vector, preserving the order of elements.

```rust
use lowdash::flatten;

let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
let flat = flatten(&nested);
assert_eq!(flat, vec![1, 2, 3, 4, 5]);
```

```rust
use lowdash::flatten;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people_groups = vec![
    vec![
        Person { name: "Alice".to_string(), age: 25 },
        Person { name: "Bob".to_string(), age: 30 },
    ],
    vec![
        Person { name: "Carol".to_string(), age: 35 },
        Person { name: "Dave".to_string(), age: 40 },
    ],
];

let flat_people = flatten(&people_groups);
assert_eq!(
    flat_people,
    vec![
        Person { name: "Alice".to_string(), age: 25 },
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Carol".to_string(), age: 35 },
        Person { name: "Dave".to_string(), age: 40 },
    ]
);
```

### interleave
Interleave multiple collections into a single vector, preserving the order of elements.

```rust
use lowdash::interleave;

let a = vec![1, 2, 3];
let b = vec![4, 5, 6, 7];
let c = vec![8, 9];

let result = interleave(&[&a[..], &b[..], &c[..]]);
assert_eq!(result, vec![1, 4, 8, 2, 5, 9, 3, 6, 7]);
```

```rust
use lowdash::interleave;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let group1 = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
];

let group2 = vec![
    Person { name: "Carol".to_string(), age: 35 },
];

let group3 = vec![
    Person { name: "Dave".to_string(), age: 40 },
    Person { name: "Eve".to_string(), age: 45 },
    Person { name: "Frank".to_string(), age: 50 },
];

let interleaved = interleave(&[&group1[..], &group2[..], &group3[..]]);
assert_eq!(
    interleaved,
    vec![
        Person { name: "Alice".to_string(), age: 25 },
        Person { name: "Carol".to_string(), age: 35 },
        Person { name: "Dave".to_string(), age: 40 },
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Eve".to_string(), age: 45 },
        Person { name: "Frank".to_string(), age: 50 },
    ]
);
```

### shuffle
Shuffle a collection, returning a new vector with the elements in random order.

```rust
use lowdash::shuffle;

let numbers = vec![1, 2, 3, 4, 5];
let shuffled = shuffle(&numbers);
assert_eq!(shuffled.len(), numbers.len());
assert!(shuffled.contains(&1));
assert!(shuffled.contains(&2));
assert!(shuffled.contains(&3));
assert!(shuffled.contains(&4));
assert!(shuffled.contains(&5));
```

### reverse
Reverse a collection, returning a new vector with the elements in reverse order.

```rust
use lowdash::reverse;

let numbers = vec![1, 2, 3, 4, 5];
let reversed = reverse(&numbers);
assert_eq!(reversed, vec![5, 4, 3, 2, 1]);
```

```rust
use lowdash::reverse;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

let reversed_people = reverse(&people);
assert_eq!(reversed_people, vec![
    Person { name: "Carol".to_string(), age: 35 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
]);
```

### fill
Fill a collection with a specified value, returning a new vector with all elements set to the initial value.

```rust
use lowdash::fill;

let numbers = vec![1, 2, 3, 4, 5];
let filled = fill(&numbers, 0);
assert_eq!(filled, vec![0, 0, 0, 0, 0]);
```

```rust
use lowdash::fill;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

let filled_people = fill(&people, Person { name: "Dave".to_string(), age: 40 });
assert_eq!(
    filled_people,
    vec![
        Person { name: "Dave".to_string(), age: 40 },
        Person { name: "Dave".to_string(), age: 40 },
        Person { name: "Dave".to_string(), age: 40 },
    ]
);
```

### repeat
Fill a collection with a specified value, returning a new vector with all elements set to the initial value.

```rust
use lowdash::repeat;

let filled = repeat(5, 0);
assert_eq!(filled, vec![0, 0, 0, 0, 0]);
```

```rust
use lowdash::repeat;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let filled_people = repeat(3, Person { name: "Dave".to_string(), age: 40 });
assert_eq!(
    filled_people,
    vec![
        Person { name: "Dave".to_string(), age: 40 },
        Person { name: "Dave".to_string(), age: 40 },
        Person { name: "Dave".to_string(), age: 40 },
    ]
);
```

### repeat_by
Repeat a specified value `count` times by applying a predicate function to each index, returning a new vector with the generated elements.

```rust
use lowdash::repeat_by;

let filled = repeat_by(5, |i| i * 2);
assert_eq!(filled, vec![0, 2, 4, 6, 8]);
```

```rust
use lowdash::repeat_by;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let filled_people = repeat_by(3, |i| Person {
    name: format!("Person {}", i + 1),
    age: 20 + i as u32 * 5,
});
assert_eq!(
    filled_people,
    vec![
        Person { name: "Person 1".to_string(), age: 20 },
        Person { name: "Person 2".to_string(), age: 25 },
        Person { name: "Person 3".to_string(), age: 30 },
    ]
);
```

### key_by
Creates a `HashMap` by mapping each element in a collection to a key using an iteratee function.

```rust
use lowdash::key_by;
use std::collections::HashMap;

let numbers = vec![1, 2, 3, 4, 5];
let map = key_by(&numbers, |&x| x % 2);
let mut expected = HashMap::new();
expected.insert(1, 5); // Last odd number
expected.insert(0, 4); // Last even number
assert_eq!(map, expected);
```

```rust
use lowdash::key_by;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Charlie".to_string(), age: 35 },
];

let map = key_by(&people, |person| person.name.clone());
let mut expected = HashMap::new();
expected.insert("Alice".to_string(), people[0].clone());
expected.insert("Bob".to_string(), people[1].clone());
expected.insert("Charlie".to_string(), people[2].clone());
assert_eq!(map, expected);
```

```rust
use lowdash::key_by;
use std::collections::HashMap;

let strings = vec!["apple", "banana", "apricot", "blueberry"];
let map = key_by(&strings, |s| s.chars().next().unwrap());
let mut expected = HashMap::new();
expected.insert('a', "apricot");
expected.insert('b', "blueberry");
assert_eq!(map, expected);
```

### associate
Creates a `HashMap` by transforming each element in a collection into a key-value pair using a provided function.

```rust
use lowdash::associate;
use std::collections::HashMap;

let numbers = vec![1, 2, 3, 4, 5];
let map = associate(&numbers, |&x| (x, x * x));
let mut expected = HashMap::new();
expected.insert(1, 1);
expected.insert(2, 4);
expected.insert(3, 9);
expected.insert(4, 16);
expected.insert(5, 25);
assert_eq!(map, expected);
```

```rust
use lowdash::associate;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Charlie".to_string(), age: 35 },
];

let map = associate(&people, |person| (person.name.clone(), person.age));
let mut expected = HashMap::new();
expected.insert("Alice".to_string(), 25);
expected.insert("Bob".to_string(), 30);
expected.insert("Charlie".to_string(), 35);
assert_eq!(map, expected);
```

```rust
use lowdash::associate;
use std::collections::HashMap;

let strings = vec!["apple", "banana", "apricot", "blueberry"];
let map = associate(&strings, |s| (s.chars().next().unwrap(), s.len()));
let mut expected = HashMap::new();
expected.insert('a', 7); // "apricot" has 7 characters
expected.insert('b', 9); // "blueberry" has 9 characters
assert_eq!(map, expected);
```

### slice_to_map
Transforms a slice of items into a `HashMap` by applying a provided function to each item.

```rust
use lowdash::slice_to_map;
use std::collections::HashMap;

let numbers = vec![1, 2, 3, 4, 5];
let map = slice_to_map(&numbers, |&x| (x, x * x));
let mut expected = HashMap::new();
expected.insert(1, 1);
expected.insert(2, 4);
expected.insert(3, 9);
expected.insert(4, 16);
expected.insert(5, 25);
assert_eq!(map, expected);
```

```rust
use lowdash::slice_to_map;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Charlie".to_string(), age: 35 },
];

let map = slice_to_map(&people, |person| (person.name.clone(), person.age));
let mut expected = HashMap::new();
expected.insert("Alice".to_string(), 25);
expected.insert("Bob".to_string(), 30);
expected.insert("Charlie".to_string(), 35);
assert_eq!(map, expected);
```

```rust
use lowdash::slice_to_map;
use std::collections::HashMap;

let strings = vec!["apple", "banana", "apricot", "blueberry"];
let map = slice_to_map(&strings, |s| (s.chars().next().unwrap(), s.len()));
let mut expected = HashMap::new();
expected.insert('a', 7); // "apricot" has 7 characters
expected.insert('b', 9); // "blueberry" has 9 characters
assert_eq!(map, expected);
```

### drop
Removes the first `n` elements from a collection and returns the remaining elements.

```rust
use lowdash::drop;

let numbers = vec![1, 2, 3, 4, 5];
let result = drop(&numbers, 2);
assert_eq!(result, vec![3, 4, 5]);
```

```rust
use lowdash::drop;

let letters = vec!['a', 'b', 'c', 'd'];
let result = drop(&letters, 10);
assert_eq!(result, vec![]);
```

### drop_right
Removes the last `n` elements from a collection and returns the remaining elements.

```rust
use lowdash::drop_right;

let numbers = vec![1, 2, 3, 4, 5];
let result = drop_right(&numbers, 2);
assert_eq!(result, vec![1, 2, 3]);
```

```rust
use lowdash::drop_right;

let letters = vec!['a', 'b', 'c', 'd'];
let result = drop_right(&letters, 10);
assert_eq!(result, vec![]);
```

### drop_while
Removes elements from the beginning of a collection as long as a predicate returns `true`, and returns the remaining elements. As soon as the predicate returns `false`, the function stops dropping elements.


```rust
use lowdash::drop_while;

let numbers = vec![1, 2, 3, 4, 5];
let result = drop_while(&numbers, |&x| x < 3);
assert_eq!(result, vec![3, 4, 5]);
```

```rust
use lowdash::drop_while;

let letters = vec!['a', 'b', 'c', 'd'];
let result = drop_while(&letters, |&c| c < 'c');
assert_eq!(result, vec!['c', 'd']);
```

### drop_right_while
Removes elements from the end of a collection as long as a predicate returns `true`, and returns the remaining elements. As soon as the predicate returns `false`, the function stops dropping elements.


```rust
use lowdash::drop_right_while;

let numbers = vec![1, 2, 3, 4, 5];
let result = drop_right_while(&numbers, |&x| x > 3);
assert_eq!(result, vec![1, 2, 3]);
```

```rust
use lowdash::drop_right_while;

let letters = vec!['a', 'b', 'c', 'd', 'e'];
let result = drop_right_while(&letters, |&c| c != 'c');
assert_eq!(result, vec!['a', 'b', 'c']);
```

### drop_by_index
Removes elements from a collection at the specified indices.
Supports negative indices which count from the end of the collection.
Indices that are out of bounds are ignored.

```rust
use lowdash::drop_by_index;

let numbers = vec![1, 2, 3, 4, 5];
let result = drop_by_index(&numbers, &[1, 3]);
assert_eq!(result, vec![1, 3, 5]);
```

```rust
use lowdash::drop_by_index;

let letters = vec!['a', 'b', 'c', 'd', 'e'];
let result = drop_by_index(&letters, &[0, -1]);
assert_eq!(result, vec!['b', 'c', 'd']);
```

### reject_map
Applies a callback function to each item in a collection along with its index and collects the results where the callback returns `false`.

```rust
use lowdash::reject_map;

let numbers = vec![1, 2, 3, 4, 5];
// Collect squares of odd numbers
let result = reject_map(&numbers, |&x, _| (x * x, x % 2 == 0));
assert_eq!(result, vec![1, 9, 25]);
```

```rust
use lowdash::reject_map;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

// Collect names of people who are not above 30
let result = reject_map(&people, |person, _| (person.name.clone(), person.age > 30));
assert_eq!(result, vec!["Alice".to_string(), "Bob".to_string()]);
```

### filter_reject
Filters a collection into two separate vectors based on a predicate function.

```rust
use lowdash::filter_reject;

let numbers = vec![1, 2, 3, 4, 5];
// Separate even and odd numbers
let (evens, odds) = filter_reject(&numbers, |&x, _| x % 2 == 0);
assert_eq!(evens, vec![2, 4]);
assert_eq!(odds, vec![1, 3, 5]);
```

```rust
use lowdash::filter_reject;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];

// Separate people who are at least 30 years old
let (adults, juniors) = filter_reject(&people, |person, _| person.age >= 30);
assert_eq!(adults, vec![
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
]);
assert_eq!(juniors, vec![
    Person { name: "Alice".to_string(), age: 25 },
]);
```

### count
Counts the number of occurrences of a specific value in a collection.

```rust
use lowdash::count;

let numbers = vec![1, 2, 2, 3, 4, 2];
let result = count(&numbers, 2);
assert_eq!(result, 3);
```

```rust
use lowdash::count;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
];

let count_alice = count(&people, Person { name: "Alice".to_string(), age: 25 });
assert_eq!(count_alice, 2);
```

### count_by 
Counts the number of elements in a collection that satisfy a given predicate.

```rust
use lowdash::count_by;

let numbers = vec![1, 2, 3, 4, 5];
// Count even numbers
let result = count_by(&numbers, |&x| x % 2 == 0);
assert_eq!(result, 2);
```

```rust
use lowdash::count_by;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
    Person { name: "Dave".to_string(), age: 30 },
];

// Count people who are at least 30 years old
let count_adults = count_by(&people, |p| p.age >= 30);
assert_eq!(count_adults, 3);
```

### count_values
Counts the number of occurrences of each value in a collection.

```rust
use lowdash::count_values;
use std::collections::HashMap;

let numbers = vec![1, 2, 2, 3, 4, 3, 5];
let result = count_values(&numbers);
let mut expected = HashMap::new();
expected.insert(1, 1);
expected.insert(2, 2);
expected.insert(3, 2);
expected.insert(4, 1);
expected.insert(5, 1);
assert_eq!(result, expected);
```

```rust
use lowdash::count_values;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Carol".to_string(), age: 35 },
];

let result = count_values(&people);
let mut expected = HashMap::new();
expected.insert(
    Person { name: "Alice".to_string(), age: 25 },
    2
);
expected.insert(
    Person { name: "Bob".to_string(), age: 30 },
    1
);
expected.insert(
    Person { name: "Carol".to_string(), age: 35 },
    1
);
assert_eq!(result, expected);
```

### count_values_by
Counts the number of occurrences of each value in a collection after applying a mapper function.

```rust
use lowdash::count_values_by;
use std::collections::HashMap;

let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
let result = count_values_by(&chars, |x| x.clone());
let mut expected = HashMap::new();
expected.insert('a', 2);
expected.insert('b', 2);
expected.insert('c', 1);
expected.insert('d', 1);
assert_eq!(result, expected);
```

```rust
use lowdash::count_values_by;
use std::collections::HashMap;

 let numbers = vec![1, 2, 2, 3, 4, 3, 5];
let result = count_values_by(&numbers, |x| *x);
let mut expected = HashMap::new();
expected.insert(1, 1);
expected.insert(2, 2);
expected.insert(3, 2);
expected.insert(4, 1);
expected.insert(5, 1);
assert_eq!(result, expected);
```

### subset
Returns a subset of the collection based on the provided offset and length.

```rust
use lowdash::subset;

let numbers = vec![1, 2, 3, 4, 5];
let result = subset(&numbers, 1, 3);
assert_eq!(result, vec![2, 3, 4]);
```

### slice
Returns a subset of the collection based on the provided start and end indices.

```rust
use lowdash::slice;

let numbers = vec![1, 2, 3, 4, 5];
let result = slice(&numbers, 1, 3);
assert_eq!(result, vec![2, 3]);
```

```rust
use lowdash::slice;

let numbers = vec![1, 2, 3, 4, 5];
let result = slice(&numbers, -3, -1);
assert_eq!(result, vec![3, 4]);
```

```rust
use lowdash::slice;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
    Person { name: "Dave".to_string(), age: 40 },
];

let result = slice(&people, 1, 3);
assert_eq!(
    result,
    vec![
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Carol".to_string(), age: 35 },
    ]
);
```

### replace
Replaces occurrences of a specified value in a collection with a new value, up to a maximum number of replacements.

```rust
use lowdash::replace;

let numbers = vec![1, 2, 2, 3, 4, 2, 5];
let result = replace(&numbers, 2, 9, 2);
assert_eq!(result, vec![1, 9, 9, 3, 4, 2, 5]);
```

```rust
use lowdash::replace;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Carol".to_string(), age: 35 },
];

let new_person = Person { name: "Dave".to_string(), age: 40 };
let result = replace(&people, people[0].clone(), new_person.clone(), 1);
assert_eq!(result, vec![new_person, people[1].clone(), people[2].clone(), people[3].clone()]);
```

### replace_all
Replaces all occurrences of a specified value in a collection with a new value.

```rust
use lowdash::replace_all;

let numbers = vec![1, 2, 2, 3, 4, 2, 5];
let result = replace_all(&numbers, 2, 9);
assert_eq!(result, vec![1, 9, 9, 3, 4, 9, 5]);
```

```rust
use lowdash::replace_all;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Carol".to_string(), age: 35 },
];

let dave = Person { name: "Dave".to_string(), age: 40 };
let result = replace_all(&people, people[0].clone(), dave.clone());
assert_eq!(
    result,
    vec![
        dave.clone(),
        people[1].clone(),
        dave.clone(),
        people[3].clone(),
    ]
);
```

### compact
Removes all zero-valued elements from a collection, preserving the order of non-zero elements.

This function iterates over a slice of items, removing each element that is equal to the zero value.
The zero value is determined by the `Default` trait implementation for the type `T`.
The function preserves the order of the remaining elements and does not modify the original collection.

**Time Complexity:** O(n), where n is the number of elements in the collection.

# Arguments

* `collection` - A slice of items from which to remove zero-valued elements.

# Type Parameters

* `T` - The type of elements in the collection. Must implement `PartialEq`, `Clone`, and `Default`.

# Returns

* `Vec<T>` - A new vector containing only the non-zero elements from the input collection.

# Examples

```rust
use lowdash::compact;

let numbers = vec![0, 1, 0, 2, 3, 0, 4];
let compacted = compact(&numbers);
assert_eq!(compacted, vec![1, 2, 3, 4]);
```

```rust
use lowdash::compact;

#[derive(Debug, PartialEq, Clone, Default)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "".to_string(), age: 0 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "".to_string(), age: 0 },
    Person { name: "Dave".to_string(), age: 40 },
];

let compacted = compact(&people);
assert_eq!(
    compacted,
    vec![
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Dave".to_string(), age: 40 },
    ]
);
```

```rust
use lowdash::compact;

let floats = vec![0.0, 1.1, 0.0, 2.2, 3.3, 0.0, 4.4];
let compacted = compact(&floats);
assert_eq!(compacted, vec![1.1, 2.2, 3.3, 4.4]);
```

### is_sorted
Determines if a collection is sorted in ascending order.

```rust
use lowdash::is_sorted;

let numbers = vec![1, 2, 3, 4, 5];
assert_eq!(is_sorted(&numbers), true);
```

```rust
use lowdash::is_sorted;

let numbers = vec![5, 4, 3, 2, 1];
assert_eq!(is_sorted(&numbers), false);
```

```rust
use lowdash::is_sorted;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];
assert_eq!(is_sorted(&people), true);
```

```rust
use lowdash::is_sorted;

let floats = vec![1.1, 2.2, 3.3, 4.4];
assert_eq!(is_sorted(&floats), true);
```

```rust
use lowdash::is_sorted;

let floats = vec![1.1, std::f64::NAN, 3.3];
// Note: Any comparison with NaN returns false, so the slice is not considered sorted.
assert_eq!(is_sorted(&floats), false);
```

### is_sorted_by_key
Determines if a collection is sorted in ascending order based on a specified key.

```rust
use lowdash::is_sorted_by_key;

let numbers = vec![
    (1, "a"),
    (2, "b"),
    (3, "c"),
    (4, "d"),
];
let result = is_sorted_by_key(&numbers, |item| item.0);
assert_eq!(result, true);
```

```rust
use lowdash::is_sorted_by_key;

let numbers = vec![
    (1, "a"),
    (3, "b"),
    (2, "c"),
    (4, "d"),
];
let result = is_sorted_by_key(&numbers, |item| item.0);
assert_eq!(result, false);
```

```rust
use lowdash::is_sorted_by_key;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
    Person { name: "Carol".to_string(), age: 35 },
];
let result = is_sorted_by_key(&people, |p| p.age);
assert_eq!(result, true);
```

```rust
use lowdash::is_sorted_by_key;

let floats = vec![
    (1.1, "apple"),
    (2.2, "banana"),
    (3.3, "cherry"),
    (4.4, "date"),
];
let result = is_sorted_by_key(&floats, |item| item.0);
assert_eq!(result, true);
```

### splice
Inserts elements into a collection at a specified index, handling negative indices and overflow.

```rust
use lowdash::splice;

let numbers = vec![1, 2, 3, 4, 5];
let elements = vec![99, 100];
let result = splice(&numbers, 2, &elements);
assert_eq!(result, vec![1, 2, 99, 100, 3, 4, 5]);
```

```rust
use lowdash::splice;

let numbers = vec![1, 2, 3, 4, 5];
let elements = vec![99, 100];
// Insert at the end
let result = splice(&numbers, 10, &elements);
assert_eq!(result, vec![1, 2, 3, 4, 5, 99, 100]);
```

```rust
use lowdash::splice;

let numbers = vec![1, 2, 3, 4, 5];
let elements = vec![99];
// Insert at negative index (-2 means len - 2 = 3)
let result = splice(&numbers, -2, &elements);
assert_eq!(result, vec![1, 2, 3, 99, 4, 5]);
```

```rust
use lowdash::splice;

let numbers = vec![1, 2, 3, 4, 5];
let elements = vec![99];
// Negative index beyond the start, insert at beginning
let result = splice(&numbers, -10, &elements);
assert_eq!(result, vec![99, 1, 2, 3, 4, 5]);
```

## 🫡 Acknowledgement
This project is inspired by [lodash](https://lodash.com/) and [lo](https://github.com/samber/lo)