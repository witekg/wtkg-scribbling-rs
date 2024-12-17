//for ref test:
use std::rc::Rc;
use std::sync::{Arc, Mutex};
//for numbers comparison:
use std::convert::TryInto;
use std::ops::Add;

fn main() {
    println!("Hello, world!");
    //greet_world();
    //ref_test();
    //declarations();
    //notations();
    fizz_buzz(25);
}

#[allow(dead_code)]
fn greet_world() {
    println!("Hello, world!"); //exclamation mark: macro
    let southern_germany = "Grüß Gott!"; // let - variable binding
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        //many types have iter() method to return iterator
        println!("{}", &region); //& means borrow value for read only
    }
}

#[allow(dead_code)]
fn ref_test() {
    let a = 10;
    let b = Box::new(20); //integer on heap, known as boxed integer
    let c = Rc::new(Box::new(30)); //boxed integer wrapped in reference counter
    let d = Arc::new(Mutex::new(40)); //Integer wrapped in an atomic reference counter and protected by a mutual exclusion lock
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}

#[allow(dead_code)]
fn declarations() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("( a + b ) + ( c + d ) = {}", e);
}

#[allow(dead_code)]
fn add(i: i32, j: i32) -> i32 {
    i + j
}

#[allow(dead_code)]
fn notations() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

#[allow(dead_code)]
fn numbers_comparison() {
    let a: i32 = 10;
    let b: u16 = 100;
    //will not compile - different types:
    /*
    if a < b {
        println!("Ten is less than one hundred.");
    }
     */
    //will compile - explicit conversion:
    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }
    //will compile - using try into trait
    let b_ = b.try_into().unwrap();//unwrap get value out of the Result
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

fn fizz_buzz(limit: i32) {
    for val in 1..limit {
        let text = fizz_buzz_value(val);
        println!("{} is {}", val, text);
    }
}

fn fizz_buzz_value(val: i32) -> String {
    let mut ret = val.to_string();
    if val % 3 == 0 {
        ret = "Fizz".to_string();
    }
    if val % 5 == 0 {
        ret = "Buzz".to_string();
    }
    if val % 3 == 0 && val % 5 == 0 {
        ret = "FizzBuzz".to_string();
    }
    ret
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn even_oo_with_match() {
    let n = 654321;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };
    println!("{} is {}", n, description);
}

fn break_return_value() {
    let n = loop {
        break 123;
    };
    println!("{}", n);//prints 123
}

fn ref_deref() {
    let a = 42;
    let r = &a; // r is reference to a
    let b = a + *r; //* means deref: get target value
    println!("a + a = {}", b);//84
}