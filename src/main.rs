#![feature(net)]
#![feature(io)]
#![feature(std_misc)]

use std::thread::Thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::net::UdpSocket;
use std::net::IpAddr;
use std::old_io::Timer;
use std::old_io::timer;
use std::time::duration::Duration;

// ================================================================================================

const CRTP_VERSION : &'static str = env!("CARGO_PKG_VERSION");

struct SSRC(u32);
struct RtpTimestamp(u32);
struct NtpTimestamp(u64);

struct SenderInfo {
  ntp_ts     : u64, // FIXME: should be NtpTimestamp,
  rtp_ts     : u32, // FIXME: should be RtpTimestamp,
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

enum RtcpPacket {
  SR(SSRC, Vec<ReportBlock>, SenderInfo),
  RR(SSRC, Vec<ReportBlock>),
  SDES(Vec<SdesChunk>),
  BYE(Vec<SSRC>, String),
}

struct CompoundRtcpPacket {
  packets : Vec<RtcpPacket>
}

struct RtpPacket;

// ================================================================================================

struct RtpSessionParameters {
  rtp_tx  :   Sender<RtpPacket>,
  rtp_rx  : Receiver<RtpPacket>,
  rtcp_tx :   Sender<CompoundRtcpPacket>,
  rtcp_rx : Receiver<CompoundRtcpPacket>
}

struct RtpSessionStatistics {
  pckt_count : u64
}

struct RtpSession {
  parameters : RtpSessionParameters,
  ssrc       : u32
}

impl RtpSession {
  pub fn new(params : RtpSessionParameters) -> RtpSession {
    RtpSession {
      parameters : params,
      ssrc       : 0    // FIXME
    }
  }

  pub fn run(&mut self) -> RtpSessionStatistics {
    let stats = RtpSessionStatistics{pckt_count : 0};
    stats
  }
}

// ================================================================================================

fn parse_rtp_packet(buf : &mut [u8], buflen : usize) -> Option<RtpPacket> {
  println!("parse_rtp_packet");
  None
}

struct RtpSocket {
  local_addr : IpAddr,
  local_port : u16
}

impl RtpSocket {
  pub fn run(&self) -> (Sender<RtpPacket>, Receiver<RtpPacket>) {
    let rx_socket = UdpSocket::bind(&(self.local_addr, self.local_port)).unwrap();
    let tx_socket = rx_socket.try_clone().unwrap();

    let (to_app, from_net) = channel::<RtpPacket>();
    let (to_net, from_app) = channel::<RtpPacket>();

    Thread::spawn(move || {
      // The receiving thread
      loop {
        let mut buf = [0; 1500];
        let (amt, src) = rx_socket.recv_from(&mut buf).unwrap();

        match parse_rtp_packet(&mut buf, amt) {
          Some(packet) => to_app.send(packet).unwrap(),
          None => {
            println!("Unable to parse packet")
          }
        }
      }
    });

    Thread::spawn(move || {
      // The sending thread
      let packet = from_app.recv().unwrap();
      // FIXME: send the packet
    });

    (to_net, from_net)
  }
}

// ================================================================================================

fn parse_sr(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  unimplemented!();
  None
}

fn parse_rr(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  unimplemented!();
  None
}

fn parse_sdes(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  unimplemented!();
  None
}

fn parse_bye(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  unimplemented!();
  None
}

fn parse_app(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  unimplemented!();
  None
}

fn parse_rtcp_packet(buf : &mut [u8], buflen : usize) -> Option<CompoundRtcpPacket> {
  if buflen < 4 {
    println!("parse_rtcp_packet: packet is too short to be RTCP");
    return None;
  }

  // FIXME: create a compound packet object

  let mut offset = 0;
  while offset != buflen {
    if offset + 3 >= buflen {
      println!("parse_rtcp_packet: packet is too short");
      return None;
    }

    let v   =   (buf[offset + 0] >> 6) & 0x03;
    let p   =  ((buf[offset + 0] >> 5) & 0x01) == 1;
    let rc  =   (buf[offset + 0] >> 0) & 0x1f;
    let pt  =    buf[offset + 1];
    let len = (((buf[offset + 2] as usize) << 8) & 0xff00) | 
              (((buf[offset + 3] as usize) << 0) & 0x0fff);

    if offset + (4 * len) > buflen {
      println!("parse_rtcp_packet: packet is too long");
      return None;
    }

    if v != 2 {
      println!("parse_rtcp_packet: version number mismatch (v={})", v);
      return None;
    }

    let packet = &buf[offset..offset + (4 * (len + 1))];

    let parsed_packet = match pt {
      200 =>   parse_sr(p, rc, len, packet),
      201 =>   parse_rr(p, rc, len, packet),
      202 => parse_sdes(p, rc, len, packet),
      203 =>  parse_bye(p, rc, len, packet),
      204 =>  parse_app(p, rc, len, packet),
      _   => {
        println!("parse_rtcp_packet: unknown packet type (pt={})", pt);
        return None;
      }
    };

    // FIXME: append parsed_packet to the compound packet

    offset += 4 + (4 * len);
  }

  None  // FIXME: return the compound packet
}

struct RtcpSocket {
  local_addr : IpAddr,
  local_port : u16
}

impl RtcpSocket {
  pub fn run(&self) -> (Sender<CompoundRtcpPacket>, Receiver<CompoundRtcpPacket>) {
    let rx_socket = UdpSocket::bind(&(self.local_addr, self.local_port)).unwrap();
    let tx_socket = rx_socket.try_clone().unwrap();

    let (to_app, from_net) = channel::<CompoundRtcpPacket>();
    let (to_net, from_app) = channel::<CompoundRtcpPacket>();

    Thread::spawn(move || {
      // The receiving thread
      loop {
        let mut buf = [0; 1500];
        let (amt, src) = rx_socket.recv_from(&mut buf).unwrap();

        match parse_rtcp_packet(&mut buf, amt) {
          Some(packet) => to_app.send(packet).unwrap(),
          None => {
            println!("Unable to parse packet")
          }
        }
      }
    });

    Thread::spawn(move || {
      // The sending thread
      let packet = from_app.recv().unwrap();
      // FIXME: send the packet
    });

    (to_net, from_net)
  }
}

// ================================================================================================

fn main() {
  println!("CRTP v{}", CRTP_VERSION);

  let rtp_socket  =  RtpSocket{local_addr: IpAddr::new_v4(0,0,0,0), local_port : 3000};
  let (rtp_tx, rtp_rx) = rtp_socket.run();

  let rtcp_socket = RtcpSocket{local_addr: IpAddr::new_v4(0,0,0,0), local_port : 3001};
  let (rtcp_tx, rtcp_rx) = rtcp_socket.run();

  let session_parameters = RtpSessionParameters {
                             rtp_tx  : rtp_tx,
                             rtp_rx  : rtp_rx,
                             rtcp_tx : rtcp_tx,
                             rtcp_rx : rtcp_rx
                           };

  let mut session = RtpSession::new(session_parameters);
  let session_statistics = session.run();

  timer::sleep(Duration::hours(1));

}

// ================================================================================================
// vim: set ts=2 sw=2 tw=0 et ai:
