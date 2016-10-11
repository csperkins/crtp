// Copyright (c) 2014-2016 University of Glasgow
// All rights reserved.
// 
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// 
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
// 
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
// 
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate byteorder;

use std::io::Cursor;
use byteorder::*;

mod timed_datagram_protocol;

// ================================================================================================

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

struct RtpSessionStatistics {
  pckt_count : u64
}

struct RtpSession {
  ssrc       : u32
}

impl RtpSession {
  pub fn new() -> RtpSession {
    RtpSession {
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

// ================================================================================================

fn parse_report_block(packet : &[u8], offset : usize) -> ReportBlock {
  ReportBlock {
    ssrc       : SSRC(BigEndian::read_u32(&packet[offset..])),
    fract_lost : packet[offset + 4],
    cumul_lost : BigEndian::read_u32(&packet[offset +  4..]) & 0x00ffffff,
    ext_seq    : BigEndian::read_u32(&packet[offset +  8..]),
    jitter     : BigEndian::read_u32(&packet[offset + 12..]),
    lsr        : BigEndian::read_u32(&packet[offset + 16..]),
    dlsr       : BigEndian::read_u32(&packet[offset + 20..]),
  }
}

fn parse_sr(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  if len < 7 {
    println!("parse_sr: packet is too short to be an SR");
    return None;
  }

  let ssrc = SSRC(BigEndian::read_u32(&packet[4..7]));
  let si   = SenderInfo {
               ntp_ts     : BigEndian::read_u64(&packet[ 8..15]),
               rtp_ts     : BigEndian::read_u32(&packet[16..19]),
               pckt_count : BigEndian::read_u32(&packet[20..23]),
               byte_count : BigEndian::read_u32(&packet[24..28])
             };

  let mut rr_list : Vec<ReportBlock> = Vec::new();
  for i in 0..rc {
    let rr = parse_report_block(packet, (28 + (i*24)) as usize);
    rr_list.push(rr);
  }

  Some(RtcpPacket::SR(ssrc, rr_list, si))
}

fn parse_rr(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  if len < 1 {
    println!("parse_sr: packet is too short to be an RR");
    return None;
  }

  let ssrc = SSRC(BigEndian::read_u32(&packet[4..7]));

  let mut rr_list : Vec<ReportBlock> = Vec::new();
  for i in 0..rc {
    let rr = parse_report_block(packet, (8 + (i*24)) as usize);
    rr_list.push(rr);
  }

  Some(RtcpPacket::RR(ssrc, rr_list))
}

fn parse_sdes(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  let mut offset = 4;
  for i in 0..rc {
    println!("sdes {}", offset);
    let mut chunk = SdesChunk {
                      ssrc  : SSRC(BigEndian::read_u32(&packet[offset..])),
                      cname : None,
                      name  : None,
                      email : None,
                      phone : None,
                      loc   : None,
                      tool  : None,
                      note  : None
                    };

    // FIXME: parse SDES chunks
    // FIXME: add chunk to the packet
  }
  None  // FIXME: return an SDES packet
}

fn parse_bye(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  unimplemented!();
}

fn parse_app(p : bool, rc : u8, len : usize, packet : &[u8]) -> Option<RtcpPacket> {
  unimplemented!();
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
        break;
      }
    };

    // FIXME: append parsed_packet to the compound packet

    offset += 4 + (4 * len);
  }

  None  // FIXME: return the compound packet
}


// ================================================================================================

fn main() {
}

// ================================================================================================
// vim: set ts=2 sw=2 tw=0 et ai:
