extern crate bincode;
extern crate sha1;
extern crate time;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate secp256k1;
extern crate sha2;
#[macro_use]
extern crate lazy_static;
extern crate bs58;
extern crate hex;
extern crate ripemd160;

pub mod block;
pub mod blocks;
pub mod display;
pub mod hash_content;
pub mod help;
pub mod identity;
pub mod message;
pub mod peers;
pub mod transaction;

use std::io::Read;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

use bincode::deserialize;

use block::Block;

use blocks::{add_block_from_message, broadcast_block, list_blocks, send_last_block_to_stream};

use peers::{create_stream, get_chain_from_stream, list_peers};

use help::list_commands;

use display::{clear_screen, get_input, set_cursor_into_input, set_cursor_into_logs};

use message::{Message, MessageLabel};

const LISTENING_PORT: &str = "10000";

/// Handle incoming TCP connections with other nodes.
///
/// Args:
///
/// `chain` - the chain to manipulate
pub fn handle_incoming_connections(chain: Arc<Mutex<Vec<Block>>>) {
    let address = format!("0.0.0.0:{}", LISTENING_PORT);
    let listener = TcpListener::bind(address).unwrap();

    /* blocks until data is received */
    for income in listener.incoming() {
        /* TODO: display message when receive a connection;
        should use mutex as it must modify the content
        of the main text area (so the cursor position
        must not be modified) */

        clear_screen();
        set_cursor_into_logs();

        let mut stream = income.unwrap();

        const MESSAGE_MAX_LENGTH: usize = 20;
        let mut buffer: Vec<u8> = vec![0; MESSAGE_MAX_LENGTH];

        /* blocks until data is received  */
        stream
            .read(&mut buffer)
            .expect("Received message is too long.");

        let message: Message = deserialize(&buffer).unwrap();
        let label = message.get_label();

        if label == &MessageLabel::AskForAllBlocks {
            send_last_block_to_stream(stream, &chain);
        } else if label == &MessageLabel::SendBlock {
            add_block_from_message(&chain, &message);
        }

        set_cursor_into_input();
    }
}
