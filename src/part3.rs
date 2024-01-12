/*
    CIS198 Homework 1
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    The Rust borrow checker may help avoid some possible bugs.

    Then answer this question:
    Q: A common source of error in swap implementations is failing to work if
       the two references are the same. Why don't you need to worry about this
       case in Rust?
    A: in rust, the compiler says `cannot borrow x1 as mutable more than once at
       a time`, so this case can be ignored in the implementation.

    (Try writing a unit test where they are both
    the same, i.e. swap_ints(&mut x, &mut x).)
*/

pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    let temp: i32 = *x1;
    *x1 = *x2;
    *x2 = temp;
}

#[test]
fn swap_ints_test() {
    let mut x1: i32 = 1;
    let mut x2: i32 = 3;
    swap_ints(&mut x1, &mut x2);
    assert_eq!(x1, 3);
    assert_eq!(x2, 1);
}

// #[test]
// fn swap_ints_test() {
//     let mut x1: i32 = 1;
//     swap_ints(&mut x1, &mut x1);
//     assert_eq!(x1, 1);
// }

/*
    Problem 2: String duplication
*/

// #[test]
// fn copy_string_test() {
//     let str1 = String::from("foo");
//     let str2 = str1;
//     assert_eq!(str1, str2);
// }
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?
// A1. when `let str2 = str1` is ran, ownership is transferred from `str1` to `str2`, meaning that
//   `str1` can no longer be used.

// Q2. How come it works fine here?
// A2. because i32s are primitive types that are copied on assignment, so `let i2 = i1` copies the
//   value of `i2` into `i1`, meaning both `i1` and `i2` can continue to be used.

#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0..times {
        v.push(String::from(s));
    }
    v
}

#[test]
fn duplicate_string_test() {
    assert_eq!(duplicate_string("123", 3), vec!["123", "123", "123"]);
    assert_eq!(duplicate_string("", 1), vec![""]);
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: /* Change in here only */ &String /* */) -> String {
    string.clone()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(/* Change in here only */ &str1 /* */));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(/* Change in here only */ &str1 /* */);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/

// fn new_ref_string<'a>() -> &'a String {
//     let s = String::new();
//     &s
// }
// not possible because the string's lifetime is only within the `new_ref_string` function, so
// returning it would result in a dangling reference. in this case, since there's no other variable
// `s` can borrow its lifetime from, it's not possible without unsafe code.

fn new_ref_str<'a>() -> &'a str {
    let s: &str = "hello world!";
    &s
}
// the reason why returning `&String` doesn't work but returning `&str` does is because `&str` has a
// static lifetime while `String` does not.

#[test]
fn new_ref_string_test() {
    assert_eq!("hello world!", new_ref_str());
}

// The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        return s1;
    }
    s2
}

#[test]
fn pick_longest2_test() {
    assert_eq!(pick_longest2("abc", "abcdef"), "abcdef");
    assert_eq!(pick_longest2("abc", ""), "abc");
}

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    If the vector is empty, return "".

    Q1. In pick_longest_in_v2, if you were to explicitly specify the lifetime
        of the input and output, what should it be?
    A1. the lifetime of the input would be to the end of the function. the 
        lifetime of the output would be the lifetime of the elements in the 
        input vector.

    Q2. What are the pros and cons of v1 and v2?
    A2. v1 is slower because it has to keep converting to and from `&str` and 
        `String`. however, it outputs a mutable `String` type that may be
        beneficial for subsequent operations. 
*/

fn pick_longest_in_v1(v: Vec<String>) -> String {
    let mut longest = String::from("");
    for s in v {
        longest = String::from(pick_longest2(&longest, &s));
    }
    longest
}

#[test]
fn pick_longest_in_v1_test() {
    let v1 = vec![String::from("abc"), String::from(""), String::from("abcde")];
    assert_eq!(pick_longest_in_v1(v1), String::from("abcde"));

    let v2 = Vec::new();
    assert_eq!(pick_longest_in_v1(v2), String::from(""));
}

fn pick_longest_in_v2(v: Vec<&str>) -> &str {
    let mut longest: &str = "";
    for s in v {
        longest = pick_longest2(longest, s);
    }
    longest
}

#[test]
fn pick_longest_in_v2_test() {
    let v1 = vec!["abc", "", "abcde"];
    assert_eq!(pick_longest_in_v2(v1), "abcde");

    let v2 = Vec::new();
    assert_eq!(pick_longest_in_v2(v2), "");
}

/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() if necessary to make any additional unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    let mut vec = v.clone();
    for i in v.len()..desired_len {
        vec.push(0);
    }
    vec
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    let mut vec = Vec::from(slice);
    for i in slice.len()..desired_len {
        vec.push(0);
    }
    vec
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    for i in v.len()..desired_len {
        v.push(0);
    }
}

#[test]
fn test_pad_with_zeros_v1() {
    assert_eq!(pad_with_zeros_v1(vec![1, 2, 3, 4], 6), vec![1, 2, 3, 4, 0, 0]);
    assert_eq!(pad_with_zeros_v1(vec![1, 2, 3, 4], 2), vec![1, 2, 3, 4]);
}

#[test]
fn test_pad_with_zeros_v2() {
    assert_eq!(pad_with_zeros_v2(&[1, 2, 3, 4], 6), &[1, 2, 3, 4, 0, 0]);
    assert_eq!(pad_with_zeros_v2(&[1, 2, 3, 4], 2), &[1, 2, 3, 4]);
}

#[test]
fn test_pad_with_zeros_v3() {
    let mut v = vec![1, 2, 3, 4];
    pad_with_zeros_v3(&mut v, 6);
    assert_eq!(v, vec![1, 2, 3, 4, 0, 0]);

    let mut v = vec![1, 2, 3, 4];
    pad_with_zeros_v3(&mut v, 2);
    assert_eq!(v, vec![1, 2, 3, 4]);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?
      because after row is appeneded to grid, it makes logical sense that the grid owns the row, 
      so it should be subsequently accessed through the grid.

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
      because it makes logical sense that when checking if something is the first row, the check
      doesn't want to change the ownership of the row. 
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row);
}

#[test]
fn append_row_test() {
    let mut v = Vec::new();
    v.push(vec![true, false, false]);
    append_row(&mut v, vec![true, true, false, true]);
    assert_eq!(v, vec![vec![true, false, false], vec![true, true, false, true]]);
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    // Check if row is the first row in grid
    // Remember to handle the case when grid is empty
    if grid.len() < 1 {
        return false;
    }
    &grid[0] == row
}

#[test]
fn is_first_row_test() {
    let mut v = Vec::new();
    v.push(vec![true, false, false]);
    v.push(vec![true, true, false, true]);
    assert_eq!(is_first_row(&v, &[true, false, false]), true);
    assert_eq!(is_first_row(&v, &[true, false]), false);
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::collections::HashMap;

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    let mut map = HashMap::new();
    for (i, s) in v {
        map.insert(
            *i, s.clone()
        );
    }
    map
}

#[test]
fn vector_to_hashmap_test() {
    assert_eq!(vector_to_hashmap(&[(1, String::from("ab")), (4, String::from("fe")), (2, String::from("0s"))]), 
               HashMap::from([(1, String::from("ab")), (4, String::from("fe")), (2, String::from("0s"))]));
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    let keys: Vec<i32> = h.iter().filter(|(key, _)| *key < &0).map(|(key, _)| *key).collect();

    for k in keys {
        h.remove(&k);
    }
}

#[test]
fn delete_negative_keys_test() {
    let mut v = HashMap::from([(1, 3), (-1, 5), (3, -4)]);
    delete_negative_keys(&mut v);
    assert_eq!(v, HashMap::from([(1, 3), (3, -4)]));
}

/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String,String>
) {
    for (key, value) in add {
        merged.entry(key)
            .and_modify(|s| s.push_str(&value))
            .or_insert(value);
    }
}

#[test]
fn merge_maps_test() {
    let mut m = HashMap::from([(String::from("asdf"), String::from("bfd")), (String::from("ajc"), String::from("4ic"))]);
    let n = HashMap::from([(String::from("hello"), String::from("world")), (String::from("asdf"), String::from("s")), (String::from("wa"), String::from("vc"))]);
    merge_maps(&mut m, n);
    assert_eq!(m, HashMap::from([(String::from("asdf"), String::from("bfds")), (String::from("ajc"), String::from("4ic")), (String::from("hello"), String::from("world")), (String::from("wa"), String::from("vc"))]));
}
