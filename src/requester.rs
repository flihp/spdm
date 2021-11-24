//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

//! A requester follows the typestate pattern
//! <https://cliffle.com/blog/rust-typestate/>
//!
//!
//! As this code is no_std, we can't use a box to minimize the size of the type
//! states. Therefore we limit the contained state, and pass in any large state
//! when needed by given parameters. We pass in parameters rather than store
//! mutable references, because we also want States to be Send, so we can use
//! them in async code outside a no_std environment.

pub mod algorithms;
pub mod capabilities;
pub mod challenge;
pub mod id_auth;
pub mod version;

mod error;

use crate::msgs::Msg;
pub use error::RequesterError;

/// Enter the first state of the Requester state machine, the `Version` state.
pub fn start() -> version::State {
    version::State {}
}

/// We expect a messsage of the given type.
///
/// Return an error if the header doesn't match the given type.
///
/// This is just an ergonomic wrapper around `Msg::parse_header` for when
/// only one message type is expected.
pub fn expect<T: Msg>(buf: &[u8]) -> Result<(), RequesterError> {
    match T::parse_header(buf) {
        Ok(true) => Ok(()),
        Ok(false) => Err(RequesterError::UnexpectedMsg {
            expected: T::NAME,
            got: buf[0],
        }),
        Err(e) => Err(e.into()),
    }
}
