#![allow(dead_code)]
// #![allow(unused_variables)]

use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    // Associated function - "Class method"
    // Self with upper S can substitute class name
    pub fn new(addr: String) -> Server {
        return Server { address: addr };
    }
    pub fn new_2(address: String) -> Self {
        Self { address }
    }
    // Methods
    // run 2 will deallocate self after getting ownership and finished call
    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        'outer: loop {
            let result = listener.accept();
            if result.is_err() {
                continue;
            }
        }
    }
    pub fn run2(&self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        'outer2: loop {
            loop {
                break 'outer2;
            }
        }
        let tup = (5, "a", listener);
    }
}
