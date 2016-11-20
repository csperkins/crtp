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

const TOKEN : mio::Token = mio::Token(0);

// =====================

struct NetworkMio {
    socket : UdpSocket
}

impl NetworkMio {
    fn new(group : &Ipv4Addr, port : u16) -> Self {
        let ip = IpAddr::V4(Ipv4Addr::new(0,0,0,0));
        let sockaddr = SocketAddr::new(ip, port);
        let socket = UdpSocket::bind(&sockaddr).unwrap();

        socket.join_multicast_v4(group, &Ipv4Addr::new(0,0,0,0)).unwrap();
        NetworkMio {
            socket : socket
        }
    }
}

impl SendDatagram for NetworkMio {
  fn send_datagram(&self, buf : &[u8], addr : SocketAddr) -> Result<usize> {
      unimplemented!();
  }
}

// =====================

struct TimersMio {
}

impl TimersMio {
    fn new() -> Self {
        TimersMio {
        }
    }
}

impl Timers for TimersMio {
  fn   start(&self, id : u32, timeout : Duration) {
      unimplemented!();
  }

  fn  cancel(&self, id : u32) {
      unimplemented!();
  }
}

// =====================

fn main() {
    let address = "224.2.2.2".parse().unwrap();
    let port    = 2223;

    let network = NetworkMio::new(&address, port);
    let timers  = TimersMio::new();
    let session = Session::<Inactive>::new(&network, &timers);

//    let active = session.join();
//    let leaving = active.leave();

    let poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);

    poll.register(&network.socket, TOKEN, Ready::readable(), PollOpt::edge()).unwrap();

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            match event.token() {
                TOKEN => {
                    println!("got event");
                }
                _ => panic!("event with unexpected token")
            }
        }
    }
}

