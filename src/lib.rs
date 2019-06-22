//! A simple networking layer for the Stahl IPC.
#![deny(
    bad_style,
    bare_trait_objects,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unions_with_drop_fields,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    while_true
)]

mod codec;
mod frame;
mod handle;
mod server;
mod types;

pub use crate::{handle::TaskHandle, server::Server, types::*};
use derive_more::{Display, From};

/// An error deserializing a datagram received from the network.
#[derive(Debug, Display, From)]
pub enum Error {
    /// An I/O error.
    Io(std::io::Error),

    /// An unexpected end of packet while parsing an incoming data packet.
    #[display(fmt = "unexpected eof")]
    UnexpectedEof,

    /// A packet had trailing data.
    #[display(fmt = "unexpected trailing data")]
    UnexpectedTrailing,

    /// An unknown frame type was encountered.
    #[display(fmt = "unknown frame type {:x}", _0)]
    UnknownFrameType(u8),
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}
