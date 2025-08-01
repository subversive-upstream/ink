// Copyright (C) Use Ink (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Utilities to call or instantiate contracts on the chain.

mod call_builder;
mod common;
mod create_builder;
mod execution;
mod selector;

/// Utility types for the cross-contract calling API.
pub mod utils {
    pub use super::{
        common::{
            ConstructorError,
            DecodeConstructorError,
            DecodeMessageResult,
            ReturnType,
            Set,
            Unset,
            Unwrap,
        },
        execution::{
            ArgsList,
            Argument,
            ArgumentList,
            ArgumentListEnd,
            EmptyArgumentList,
        },
    };
}

pub use self::{
    call_builder::{
        build_call,
        build_call_abi,
        build_call_solidity,
        Call,
        CallBuilder,
        CallParams,
        DelegateCall,
    },
    create_builder::{
        build_create,
        build_create_abi,
        build_create_solidity,
        state,
        ConstructorReturnType,
        CreateBuilder,
        CreateParams,
        FromAddr,
        LimitParamsV2,
    },
    execution::{
        Execution,
        ExecutionInput,
        Executor,
    },
    selector::Selector,
};
