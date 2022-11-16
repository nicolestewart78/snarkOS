// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

mod common;
use common::TestPeer;

use snarkos_node::new_beacon::Beacon;
use snarkos_node_executor::NodeType;
use snarkvm::prelude::{ConsensusMemory, PrivateKey, Testnet3 as CurrentNetwork};

use std::str::FromStr;

use pea2pea::Pea2Pea;

#[tokio::test]
async fn handshake_responder_side() {
    // Create a beacon instance.
    let beacon = Beacon::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::new(
        "127.0.0.1:4133".parse().unwrap(),
        None,
        PrivateKey::<CurrentNetwork>::from_str("APrivateKey1zkp2oVPTci9kKcUprnbzMwq95Di1MQERpYBhEeqvkrDirK1").unwrap(),
        &[],
        None, // Should load the current network's genesis block.
        None,
    )
    .await
    .expect("couldn't create beacon instance");

    // Spin up a test peer.
    let peer = TestPeer::new(NodeType::Validator).await;

    // Verify the handshake works when the peer initates a connection with the node.
    assert!(
        peer.node()
            .connect(beacon.router().network().listening_addr().expect("beacon listener should exist"))
            .await
            .is_ok()
    );
}
