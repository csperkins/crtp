// Copyright (c) 2016 University of Glasgow
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

use std::io::Result;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

pub struct Inactive;
pub struct Active;
pub struct Leaving;

pub trait SendDatagram {
    fn send_datagram(&self, buf : &[u8], addr : SocketAddr) -> Result<usize>;
}

pub enum TimerId {
    RtcpTimer
}

pub trait Timeout {
    fn    set_timeout(&mut self, timeout_after : Duration, id : TimerId);
    fn cancel_timeout(&mut self, id : TimerId);
}

pub struct Session<'a, State, > {
    network : &'a SendDatagram,
    timeout : &'a Timeout,
    state   : State,
    ssrc    : u32
}

// ================================================================================================

impl <'a> Session<'a, Inactive> {
    pub fn new(network: &'a SendDatagram, timeout: &'a Timeout) -> Session<'a, Inactive> {
        Session {
            network : network,
            timeout : timeout,
            state   : Inactive,
            ssrc    : 0             // FIXME
        }
    }

    pub fn join(self) -> Session<'a, Active> {
        unimplemented!();
    }
}

// ================================================================================================

impl <'a> Session<'a, Active> {
    pub fn leave(self) -> Session<'a, Leaving> {
        unimplemented!();
    }

    fn recv_datagram(&self, now : Instant, buf : &[u8], addr : SocketAddr) {
        unimplemented!();
    }

    fn timeout(&self, now : Instant, timer : u32) {
        unimplemented!();
    }
}

// ================================================================================================

impl <'a> Session<'a, Leaving> {
    fn recv_datagram(&self, now : Instant, buf : &[u8], addr : SocketAddr) {
        unimplemented!();
    }

    fn timeout(&self, now : Instant, timer : u32) {
        unimplemented!();
    }
}

// ================================================================================================
