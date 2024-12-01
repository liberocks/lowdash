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
- [earliest_by](#earliest_by)
- [earliest](#earliest)
- [find_duplicates_by](#find_duplicates_by)
- [find_duplicates](#find_duplicates)
- [find_index_of](#find_index_of)
- [find_key_by](#find_key_by)
- [find_key](#find_key)
- [find_last_index_of](#find_last_index_of)
- [find_or_else](#find_or_else)
- [find_uniques_by](#find_uniques_by)
- [find_uniques](#find_uniques)
- [find](#find)
- [first_or_empty](#first_or_empty)
- [first_or](#first_or)
- [first](#first)
- [index_of](#index_of)
- [last_index_of](#last_index_of)
- [last_or_empty](#last_or_empty)
- [last_or](#last_or)
- [last](#last)
- [latest_by](#latest_by)
- [latest](#latest)
- [max_by](#max_by)
- [max](#max)
- [min_by](#min_by)
- [min](#min)
- [nth](#nth)
- [sample](#sample)
- [samples](#samples)

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

## 🫡 Acknowledgement
This project is inspired by [lodash](https://lodash.com/) and [lo](https://github.com/samber/lo)