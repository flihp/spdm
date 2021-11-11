use crate::crypto::digest::Digest;

// TODO: Don't Hardcode these sizes
//
// It would be great if we could make these associated constants in `Config` but
// unfortunately, we need these for array sizes, and associated constants don't
// play well with that use case or const generics.
//
// We can use associated constants with const generics with
// `#![feature(const_evaluatable_checked)]` but that requires nightly.
// See https://github.com/rust-lang/rust/issues/76560

// The number of stored certificate chains used in the system. There can
// be up to 8 slots.
//
// While a responder can have more slots than this in use, the requester
// will only store information and utilize the first NUM_SLOTS.
pub const NUM_SLOTS: usize = 1;

// The maximum size of a certificate chain supported in the system. The
// absolute maximum size supported by the spec is 65536 bytes.
pub const MAX_CERT_CHAIN_SIZE: usize = 1536;

// This must be larger than MAX_CERT_CHAIN_SIZE
pub const TRANSCRIPT_SIZE: usize = 2048;

pub trait Config {
    type Digest: Digest;
}