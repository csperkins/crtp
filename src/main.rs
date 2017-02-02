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
extern crate mio;

mod packets;
mod session;

use session::*;
use std::io::Result;
use std::str::FromStr;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use mio::*;
use mio::udp::*;
use mio::timer::Timer;

const SOCKET_TOKEN : mio::Token = mio::Token(0);
const  TIMER_TOKEN : mio::Token = mio::Token(1);

// =====================

impl SendDatagram for UdpSocket {
  fn send_datagram(&self, buf : &[u8], addr : SocketAddr) -> Result<usize> {
      match self.send_to(buf, &addr) {
          Ok(None) => {
              Ok(0)     // EWOULDBLOCK
          }
          Ok(Some(size)) => {
              Ok(size)
          }
          Err(e) => {
              Err(e)
          }
      }
  }
}

// =====================

impl Timeout for Timer<TimerId> {
    fn set_timeout(&mut self, timeout_after : Duration, id : TimerId) {
        unimplemented!();
    }

    fn cancel_timeout(&mut self, id : TimerId) {
        unimplemented!();
    }
}

// =====================

fn main() {
    let port    = 2223;

    let ip = IpAddr::V4(Ipv4Addr::new(0,0,0,0));
    let sockaddr = SocketAddr::new(ip, port);
    let socket = UdpSocket::bind(&sockaddr).unwrap();

    socket.join_multicast_v4(&Ipv4Addr::new(224,2,2,2), &Ipv4Addr::new(0,0,0,0)).unwrap();

    let mut timer = timer::Builder::default().tick_duration(Duration::from_millis(1)).build();
//    let session = Session::<Inactive>::new(&socket, &timer);

//    let active = session.join();
//    let leaving = active.leave();

    let poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);

    poll.register(&socket, SOCKET_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();
    poll.register(&timer,   TIMER_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();

    timer.set_timeout(Duration::from_millis(5000), TimerId::RtcpTimer);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            match event.token() {
                SOCKET_TOKEN => {
                    println!("got socket event");
                }
                TIMER_TOKEN => {
                    println!("got timer event");
                    timer.set_timeout(Duration::from_millis(5000), TimerId::RtcpTimer);
                }
                _ => panic!("event with unexpected token")
            }
        }
    }
}

