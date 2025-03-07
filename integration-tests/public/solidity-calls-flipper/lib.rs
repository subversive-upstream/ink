#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract(abi = "sol")]
pub mod flipper {
    use crate::keccak_selector;
    use ink::{
        env::{
            call::{
                build_call_solidity,
                ExecutionInput,
            },
            CallFlags,
        },
        H160,
    };
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        // solidity compatible selector (`keccack256("flip()")`)
        #[ink(message, selector = 0xcde4efa9)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn set(&mut self, value: bool) {
            self.value = value;
        }

        #[ink(message)]
        pub fn flip_2(&mut self) {
            self.value = !self.value;
        }

        /// Returns the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        // solidity compatible selector (`keccack256("get_2()")`)
        #[ink(message, selector = 0x6d4ce63c)]
        pub fn get_2(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn call_solidity_set(&mut self, callee: [u8; 20]) {
            let selector = keccak_selector(b"set_value(uint16)");
            let callee: H160 = callee.into();

            let result = build_call_solidity::<<Self as ::ink::env::ContractEnv>::Env>()
                .call(callee)
                .ref_time_limit(1000000000)
                .transferred_value(ink::U256::zero())
                .call_flags(CallFlags::empty())
                .exec_input(ExecutionInput::new(selector.into()).push_arg(77u16))
                .returns::<()>()
                .try_invoke();

            assert!(result.is_ok(), "call failed");
        }

        #[ink(message)]
        pub fn call_solidity_get(&mut self, callee: [u8; 20]) -> u16 {
            let selector = crate::keccak_selector(b"get_value()");
            let callee: H160 = callee.into();

            let result = build_call_solidity::<<Self as ::ink::env::ContractEnv>::Env>()
                .call(callee)
                .ref_time_limit(1000000000)
                .transferred_value(ink::U256::zero())
                .call_flags(CallFlags::empty())
                .exec_input(ExecutionInput::new(selector.into()))
                .returns::<u16>()
                .invoke();

            result
        }
    }
}

fn keccak_selector(input: &[u8]) -> [u8; 4] {
    let mut output = [0; 32];
    use sha3::{
        digest::generic_array::GenericArray,
        Digest as _,
    };
    let mut hasher = sha3::Keccak256::new();
    hasher.update(input);
    hasher.finalize_into(<&mut GenericArray<u8, _>>::from(&mut output[..]));
    [output[0], output[1], output[2], output[3]]
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;
