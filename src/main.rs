// ================================================================================================

type SSRC         = u32;
type RtpTimestamp = u32;
type NtpTimestamp = u64;

#[deriving(Clone)]
struct SenderInfo {
  ntp_ts : NtpTimestamp,
  rtp_ts : RtpTimestamp,
  pckt_count : u32,
  byte_count : u32
}

#[deriving(Clone)]
struct ReportBlock {
  ssrc       : SSRC,
  fract_lost : u8,
  cumul_lost : u32,
  ext_seq    : u32,
  jitter     : u32,
  lsr        : u32,
  dlsr       : u32
}

#[deriving(Clone)]
struct SdesItem {
  item_type : u8,
  item_text : String
}

#[deriving(Clone)]
struct SdesChunk {
  ssrc  : SSRC,
  items : Vec<SdesItem>
}

#[deriving(Clone)]
enum PacketRTCP {
  PacketSR(SSRC, Vec<ReportBlock>, SenderInfo),
  PacketRR(SSRC, Vec<ReportBlock>),
  PacketSDES(Vec<SdesChunk>),
  PacketBye(Vec<SSRC>, String),
}

#[deriving(Clone)]
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
