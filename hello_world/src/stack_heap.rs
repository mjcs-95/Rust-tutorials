#![allow(dead_code)]

use std::mem;



// Stack: is used to store local variables of the current function

// Heap: is a region of the process manually managed, 
//       global scope in the program, by default rust uses smart pointers


struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    // stack is fast but has limited size
    // heap for longer term storage
    let p1 = origin(); // to stack
    let p2 = Box::new(origin()); // store a pointer in stack to heap position
    println!("p1 takes up to {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up to {} bytes", mem::size_of_val(&p2));
    let p3 = *p2;
    println!("p3 takes up to {} bytes", mem::size_of_val(&p3));
}


