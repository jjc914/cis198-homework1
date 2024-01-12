/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

pub fn double_v1(n: i32) -> i32 {
    n + n
}

pub fn double_v2(n: &i32) -> i32 {
    // i think i prefer this one because it allows the variable continued use after borrowing
    // within the caller function
    n + n
}

pub fn double_v3(n: &mut i32) {
    *n = *n + *n;
}

#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
    assert_eq!(double_v1(0), 0);
}

#[test]
fn test_double_v2() {
    assert_eq!(double_v2(&2), 4);
    assert_eq!(double_v2(&-3), -6);
    assert_eq!(double_v2(&0), 0);
}

#[test]
fn test_double_v3() {
    let mut n: i32 = 2;
    double_v3(&mut n);
    assert_eq!(n, 4);

    n = -3;
    double_v3(&mut n);
    assert_eq!(n, -6);

    n = 0;
    double_v3(&mut n);
    assert_eq!(n, 0);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/

pub fn sqrt_v1(n: usize) -> usize {
    for i in 0_usize..(n + 1) {
        if (i + 1) * (i + 1) > n {
            return i;
        }
    }
    0
}

pub fn sqrt_v2(n: usize) -> usize {
    let mut change: usize = n / 2;
    let mut guess: usize = n;
    while change > 1 {
        if guess * guess < n {
            guess += change;
        } else if guess * guess > n {
            guess -= change;
        } else {
            return guess;
        }
        change /= 2;
    }
    guess
}

#[test]
fn test_sqrt_v1() {
    assert_eq!(sqrt_v1(64), 8);
    assert_eq!(sqrt_v1(49), 7);
    assert_eq!(sqrt_v1(63), 7);
    assert_eq!(sqrt_v1(0), 0);
    assert_eq!(sqrt_v1(1), 1);
    assert_eq!(sqrt_v1(8), 2);
    assert_eq!(sqrt_v1(10), 3);
}

#[test]
fn test_sqrt_v2() {
    assert_eq!(sqrt_v2(64), 8);
    assert_eq!(sqrt_v2(49), 7);
    assert_eq!(sqrt_v2(63), 7);
    assert_eq!(sqrt_v2(0), 0);
    assert_eq!(sqrt_v2(1), 1);
    assert_eq!(sqrt_v2(8), 2);
    assert_eq!(sqrt_v2(10), 3);
}

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
*/

pub fn sum_v1(slice: &[i32]) -> i32 {
    // i prefer this way because there's no need to dereference every time i use the variable
    let mut sum: i32 = 0;
    for &v in slice {
        sum += v;
    }
    sum
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for v in slice {
        sum += *v;
    }
    sum
}

#[test]
fn test_sum_v1() {
    assert_eq!(sum_v1(&[1, 3, 5, 2, 0]), 11);
    assert_eq!(sum_v1(&[3, 2]), 5);
    assert_eq!(sum_v1(&[1]), 1);
    assert_eq!(sum_v1(&[0]), 0);
    assert_eq!(sum_v1(&[0, 0]), 0);
    assert_eq!(sum_v1(&[0, 0, 0]), 0);
}

#[test]
fn test_sum_v2() {
    assert_eq!(sum_v2(&[1, 3, 5, 2, 0]), 11);
    assert_eq!(sum_v2(&[3, 2]), 5);
    assert_eq!(sum_v2(&[1]), 1);
    assert_eq!(sum_v2(&[0]), 0);
    assert_eq!(sum_v2(&[0, 0]), 0);
    assert_eq!(sum_v2(&[0, 0, 0]), 0);
}

/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut vec = Vec::new();
    for &v in slice {
        let mut contains: bool = false;
        for &v2 in &vec {
            if v == v2 {
                contains = true;
                break;
            }
        }
        if !contains {
            vec.push(v);
        }
    }
    vec
}

#[test]
fn test_unique() {
    assert_eq!(unique(&[1, 3, 5, 2, 0]), vec![1, 3, 5, 2, 0]);
    assert_eq!(unique(&[1, 3, 1, 2, 0]), vec![1, 3, 2, 0]);
    assert_eq!(unique(&[0]), vec![0]);
    assert_eq!(unique(&[0, 0]), vec![0]);
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/

pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut vec = Vec::new();
    for &v in slice {
        if pred(v) {
            vec.push(v);
        }
    }
    vec
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/

pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let mut vec = Vec::new();
    if out_size == 0 {
        return vec;
    }
    if out_size == 1 {
        vec.push(n1);
        return vec;
    }
    vec.push(n1);
    vec.push(n2);
    if out_size == 2 {
        return vec;
    }
    for i in 0..(out_size - 2) {
        vec.push(vec[i] + vec[i+1]);
    }
    vec
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(1, 1, 5), vec![1, 1, 2, 3, 5]);
    assert_eq!(fibonacci(2, 3, 1), vec![2]);
    assert_eq!(fibonacci(2, 3, 2), vec![2, 3]);
    assert_eq!(fibonacci(2, 3, 4), vec![2, 3, 5, 8]);
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
*/

pub fn str_concat(s1: &str, s2: &str) -> String {
    s1.to_owned() + s2
}

pub fn string_concat(s1: String, s2: String) -> String {
    // may be less efficient because:
    //   1. string method ownership needs to be taken, whereas str method operates on the
    //      references, which doesnt require ownership transfer
    //   2. str method has known length at compile time, allowing for known length of the resulting
    //      string at compile time, leading to no unnecessary allocations or reallocations
    //   3. string method has unknown length at compile time, meaning there may be unnecessary
    //      allocations or reallocations when adding s2 to s1 if s1 does not have enough capacity
    //      for s2
    let mut s: String = s1;
    s.push_str(&s2);
    s
}

#[test]
fn test_str_concat() {
    assert_eq!(str_concat("asd", "fgh"), "asdfgh");
    assert_eq!(str_concat("", "fgh"), "fgh");
    assert_eq!(str_concat("asd", ""), "asd");
}

#[test]
fn test_string_concat() {
    assert_eq!(string_concat(String::from("asd"), String::from("fgh")), String::from("asdfgh"));
    assert_eq!(string_concat(String::from(""), String::from("fgh")), String::from("fgh"));
    assert_eq!(string_concat(String::from("asd"), String::from("")), String::from("asd"));
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut s = String::from("");
    for string in v {
        s.push_str(&string);
    }
    s
}

#[test]
fn test_concat_all() {
    assert_eq!(concat_all(vec![String::from("as"), String::from("df"), String::from("gh")]), String::from("asdfgh"));
    assert_eq!(concat_all(vec![String::from("as"), String::from(""), String::from("dfgh")]), String::from("asdfgh"));
    assert_eq!(concat_all(vec![String::from("")]), String::from(""));
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut vec = Vec::new();
    for s in v {
        vec.push(s.parse::<i32>().expect("ignoring error"));
    }
    vec
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut vec = Vec::new();
    for i in v {
        vec.push(format!("{}", i));
    }
    vec
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    let mut v = fibonacci(1, 1, n);
    fn is_even(val: i32) -> bool {
        val % 2 == 0
    }
    v = filter(&v, &is_even);
    concat_all(print_all(v))
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
