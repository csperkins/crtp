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

use std::net::ToSocketAddrs;
use std::time::{Duration, Instant};

use timed_datagram_protocol::*;

pub struct Inactive;
pub struct Active;
pub struct Leaving;

pub struct Session<State> {
    state : State,
    ssrc  : u32
}

// ================================================================================================

impl Session<Inactive> {
    pub fn new() -> Session<Inactive> {
        unimplemented!();
    }

    pub fn join(self) -> Session<Active> {
        unimplemented!();
    }
}

// ================================================================================================

impl Session<Active> {
    pub fn leave(self) -> Session<Leaving> {
        unimplemented!();
    }
}

impl TimedDatagramProtocolRecv for Session<Active> {
    fn recv_datagram<A : ToSocketAddrs>(&self, now : Instant, buf : &[u8], addr : A) {
    }

    fn timeout(&self, now : Instant, id : u32) {
    }
}

// ================================================================================================

impl Session<Leaving> {
}

impl TimedDatagramProtocolRecv for Session<Leaving> {
    fn recv_datagram<A : ToSocketAddrs>(&self, now : Instant, buf : &[u8], addr : A) {
    }

    fn timeout(&self, now : Instant, id : u32) {
    }
}

// ================================================================================================
