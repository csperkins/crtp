extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::{Encodable, Encoder};

// ================================================================================================

const CRTP_VERSION : &'static str = env!("CARGO_PKG_VERSION");

struct SSRC(u32);
struct RtpTimestamp(u32);
struct NtpTimestamp(u64);

struct SenderInfo {
  ntp_ts     : u32, // FIXME: should be NtpTimestamp,
  rtp_ts     : u32, // FIXME: should be RtpTimestamp,
  pckt_count : u32,
  byte_count : u32
}

impl Encodable for SenderInfo {
  fn encode<S : Encoder>(&self, encoder : &mut S) -> Result<(), S::Error> {
    encoder.emit_u32(self.ntp_ts)
  }
}

struct ReportBlock {
  ssrc       : SSRC,
  fract_lost : u8,
  cumul_lost : u32,
  ext_seq    : u32,
  jitter     : u32,
  lsr        : u32,
  dlsr       : u32
}

struct SdesChunk {
  ssrc  : SSRC,
  cname : Option<String>,
  name  : Option<String>,
  email : Option<String>,
  phone : Option<String>,
  loc   : Option<String>,
  tool  : Option<String>,
  note  : Option<String>
}

enum PacketRTCP {
  SR(SSRC, Vec<ReportBlock>, SenderInfo),
  RR(SSRC, Vec<ReportBlock>),
  SDES(Vec<SdesChunk>),
  BYE(Vec<SSRC>, String),
}

enum Packet {
  PacketRTP,
  PacketCompoundRTCP(Vec<PacketRTCP>)
}

// ================================================================================================

fn main() {
  println!("CRTP v{}", CRTP_VERSION)
}

// ================================================================================================
// vim: set ts=2 sw=2 tw=0 et ai:
