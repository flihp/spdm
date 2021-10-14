use core::convert::From;

use super::algorithms;
use crate::msgs::capabilities::{ReqFlags, RspFlags};
use crate::msgs::{Algorithms, VersionEntry};

// After the negotiation state, the requester has to identify the responder.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub version: VersionEntry,
    pub requester_ct_exponent: u8,
    pub requester_cap: ReqFlags,
    pub responder_ct_exponent: u8,
    pub responder_cap: RspFlags,
    pub algorithms: Algorithms
}

impl From<algorithms::State> for State {
    fn from(s: algorithms::State) -> Self {
        State {
            version: s.version,
            requester_ct_exponent: s.requester_ct_exponent,
            requester_cap: s.requester_cap,
            responder_ct_exponent: s.responder_ct_exponent,
            responder_cap: s.responder_cap,
            algorithms: s.algorithms.unwrap()
        }
    }
}
