//! Relight wire protocol.
//!
//! `generated/` is produced by `pnpm codegen` from `schemas/*.fbs`; never edit
//! it by hand. This crate adds the protocol version, envelope builders and a
//! cheap header reader for routing.

#[allow(
    clippy::all,
    clippy::pedantic,
    unsafe_code,
    unused_imports,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    mismatched_lifetime_syntaxes
)]
#[rustfmt::skip]
#[path = "generated/envelope_generated.rs"]
mod envelope_generated;

pub use envelope_generated::relight::protocol as fb;

use flatbuffers::FlatBufferBuilder;
use thiserror::Error;

/// Bump on every schema change; keep in sync with `packages/protocol`.
/// Clients must tolerate `PROTOCOL_VERSION - 1`.
pub const PROTOCOL_VERSION: u8 = 1;

#[derive(Debug, Error, PartialEq)]
pub enum ProtocolError {
    #[error("not a Relight envelope (missing file identifier)")]
    BadIdentifier,
    #[error("malformed envelope: {0}")]
    Invalid(#[from] flatbuffers::InvalidFlatbuffer),
}

/// The routing fields of an envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub version: u8,
    pub kind: fb::Payload,
    pub tick: u32,
}

/// Verifies `bytes` and returns the envelope.
///
/// # Errors
/// Fails if the buffer lacks the `RLGT` identifier or does not verify.
pub fn decode(bytes: &[u8]) -> Result<fb::Envelope<'_>, ProtocolError> {
    // Root offset (4 bytes) + file identifier (4 bytes); the generated check
    // panics on shorter buffers.
    if bytes.len() < 8 || !fb::envelope_buffer_has_identifier(bytes) {
        return Err(ProtocolError::BadIdentifier);
    }
    Ok(fb::root_as_envelope(bytes)?)
}

/// Reads only the routing header.
///
/// # Errors
/// Same as [`decode`].
pub fn read_header(bytes: &[u8]) -> Result<Header, ProtocolError> {
    let env = decode(bytes)?;
    Ok(Header {
        version: env.version(),
        kind: env.payload_type(),
        tick: env.tick(),
    })
}

fn finish(
    mut fbb: FlatBufferBuilder<'_>,
    tick: u32,
    kind: fb::Payload,
    payload: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>,
) -> Vec<u8> {
    let env = fb::Envelope::create(
        &mut fbb,
        &fb::EnvelopeArgs {
            version: PROTOCOL_VERSION,
            tick,
            payload_type: kind,
            payload: Some(payload),
        },
    );
    fb::finish_envelope_buffer(&mut fbb, env);
    fbb.finished_data().to_vec()
}

pub fn encode_ping(tick: u32, nonce: u32) -> Vec<u8> {
    let mut fbb = FlatBufferBuilder::new();
    let ping = fb::Ping::create(&mut fbb, &fb::PingArgs { nonce });
    finish(fbb, tick, fb::Payload::Ping, ping.as_union_value())
}

pub fn encode_pong(tick: u32, nonce: u32) -> Vec<u8> {
    let mut fbb = FlatBufferBuilder::new();
    let pong = fb::Pong::create(&mut fbb, &fb::PongArgs { nonce });
    finish(fbb, tick, fb::Payload::Pong, pong.as_union_value())
}

pub fn encode_subscribe(channels: &[&str]) -> Vec<u8> {
    let mut fbb = FlatBufferBuilder::new();
    let names: Vec<_> = channels.iter().map(|c| fbb.create_string(c)).collect();
    let channels = fbb.create_vector(&names);
    let sub = fb::Subscribe::create(
        &mut fbb,
        &fb::SubscribeArgs {
            channels: Some(channels),
        },
    );
    finish(fbb, 0, fb::Payload::Subscribe, sub.as_union_value())
}

#[cfg(test)]
mod tests {
    use super::{decode, encode_ping, encode_subscribe, fb, read_header, ProtocolError};

    #[test]
    fn ping_round_trips() {
        let bytes = encode_ping(7, 42);
        let header = read_header(&bytes).unwrap();
        assert_eq!(header.version, super::PROTOCOL_VERSION);
        assert_eq!(header.kind, fb::Payload::Ping);
        assert_eq!(header.tick, 7);
        let env = decode(&bytes).unwrap();
        assert_eq!(env.payload_as_ping().unwrap().nonce(), 42);
    }

    #[test]
    fn subscribe_round_trips() {
        let bytes = encode_subscribe(&["strategic", "island:vesper"]);
        let env = decode(&bytes).unwrap();
        let channels: Vec<&str> = env
            .payload_as_subscribe()
            .unwrap()
            .channels()
            .unwrap()
            .iter()
            .collect();
        assert_eq!(channels, ["strategic", "island:vesper"]);
    }

    #[test]
    fn rejects_garbage() {
        assert_eq!(
            read_header(b"definitely not flatbuffers").unwrap_err(),
            ProtocolError::BadIdentifier
        );
        assert!(read_header(&[]).is_err());
    }

    /// `schemas/fixtures/ping.bin` is decoded by the TS tests to prove both
    /// languages agree on the wire format. Regenerate with
    /// `UPDATE_FIXTURES=1 cargo test -p protocol`.
    #[test]
    fn ping_fixture_is_current() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schemas/fixtures/ping.bin"
        );
        let bytes = encode_ping(7, 42);
        if std::env::var_os("UPDATE_FIXTURES").is_some() {
            std::fs::write(path, &bytes).unwrap();
        }
        let fixture = std::fs::read(path).expect("fixture exists");
        assert_eq!(
            fixture, bytes,
            "run UPDATE_FIXTURES=1 cargo test -p protocol"
        );
    }
}
