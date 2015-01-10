// ================================================================================================

const CRTP_VERSION : &'static str = env!("CARGO_PKG_VERSION");

struct SSRC(u32);
struct RtpTimestamp(u32);
struct NtpTimestamp(u64);

struct SenderInfo {
  ntp_ts     : NtpTimestamp,
  rtp_ts     : RtpTimestamp,
  pckt_count : u32,
  byte_count : u32
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
  println!("CRTP")
}

// ================================================================================================
// vim: set ts=2 sw=2 tw=0 et ai:
