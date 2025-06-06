#![allow(unexpected_cfgs)]

#[ink::contract]
mod contract {
    #[ink(storage)]
    pub struct Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn constructor() -> Self {
            Self {}
        }

        #[ink(message, selector = 0xCAFEBABA, selector = _)]
        pub fn message(&self) {}
    }
}

fn main() {}
