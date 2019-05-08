//
// Created on Wed May 08 2019
//
// Copyright (c) 2019 Cryptoeconomics Lab, Inc.
// This file is part of Plasma Chamber.
//

extern crate jsonrpc_core;
extern crate jsonrpc_derive;

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

/// Plasma JSON RPC trait.
#[rpc]
pub trait PlasmaRpc {
    /// Returns a protocol version
    #[rpc(name = "protocolVersion")]
    fn protocol_version(&self) -> Result<String>;
}
