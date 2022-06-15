// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
#![allow(dead_code)]

use std::mem;

use std::env;
use std::str::FromStr; // useful functions and types for interacting with the
                       // execution environment, like args for command line arguments

mod stack_heap;

fn core_data_types_8() {
    let a: u8 = 123; //unsigned of 8 bits, let makes a inmutable/const
    println!("a = {}", a);
    let mut b: i8 = 0; //mut allows us to modify the value
    println!("b = {} before", b);
    b = -126;
    println!("b = {} after", b);
    // u = unsigned, 0 to (2^n)-1 || i = signed, -(2^(n-1)) .. (2^(N-1))-1
    let mut c = 123456789; // i32 = 32 b = 4B
    println!("c = {}, takes up to {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);
    // u[8,16,32,64],i[8,16,32,64]
    // usize, isize to use the system nº of bits
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z); // usize
    println!(
        "z = {}, size_of_z = {}, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );
    let d: char = 'x'; // '' for char "" for strings
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));
    // f32 and f64 from IEEE754 signed!
    let e: f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));
    let g: bool = true;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators_9() {
    let mut a = 2 * 3 + 5;
    println!("{}", a);
    a += 1; //Rust doest not support  ++/--
    println!("{}", a);
    println!("remainder of {} / {} = {}", a, 3, (a % 3));
    let a_cubed = i32::pow(a, 3); // Rust does not have pow operator like **
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {} to pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | or & and  ^ xor ! not, 01 or 10 = 11 == 3_10
    println!("1 | 2 = {}", c);
    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    println!("PI < 4 = {}", pi_less_4); // > <= >= == !=
    let x = 6;
    let x_is_5 = x == 5;
    println!("{} == 5 is {}", x, x_is_5);
}

fn scope_and_shadowing_10() {
    let a = 1.0;
    println!("old a = {}", a);
    let a = 123; // we can redeclare variables even with new types
    {
        println!("outer a = {}", a);
        let b = 456;
        println!("b = {}", b);
        let a = 789;
        println!("scoped new a = {}", a);
    }
    println!("outside a = {}", a);
    // println!("b = {}", b); // scope error
}

const MEANING_OF_LIFE: u8 = 42; //no fixed address, so value wil be copied
static Z: i32 = 123; // assigned var
static mut Y: i32 = 456; // assigned var and requires unsafe

fn declaring_and_using_constants_11() {
    println!("{}", MEANING_OF_LIFE);
    println!("{}", Z);
    unsafe {
        // Im telling the compiler to trust me
        println!("{}", Y); 
    }
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn greatest_common_divisors() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn main() {
    

}
