use std::env;
use std::fmt::Display;

extern crate rhai;
extern crate rust_blockchain;
use rhai::{Engine, RegisterFn};

fn showit<T: Display>(x: &mut T) -> () {
    println!("{}", x)
}

fn main() {
    for fname in env::args().skip(1) {
        let mut engine = Engine::new();

        engine.register_fn("print", showit as fn(x: &mut i32) -> ());
        engine.register_fn("print", showit as fn(x: &mut i64) -> ());
        engine.register_fn("print", showit as fn(x: &mut u32) -> ());
        engine.register_fn("print", showit as fn(x: &mut u64) -> ());
        engine.register_fn("print", showit as fn(x: &mut f32) -> ());
        engine.register_fn("print", showit as fn(x: &mut f64) -> ());
        engine.register_fn("print", showit as fn(x: &mut bool) -> ());
        engine.register_fn("print", showit as fn(x: &mut String) -> ());

        match engine.eval_file::<()>(&fname) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
    }
}

use std::sync::{Arc, Mutex};
use std::thread::spawn;

use rust_blockchain::handle_incoming_connections;
use rust_blockchain::block::Block;
use rust_blockchain::blocks::{add_block_from_message, broadcast_block, list_blocks, send_last_block_to_stream};
use rust_blockchain::peers::{create_stream, get_chain_from_stream, list_peers};
use rust_blockchain::help::list_commands;
use rust_blockchain::display::{clear_screen, get_input, set_cursor_into_input, set_cursor_into_logs};
use rust_blockchain::message::{Message, MessageLabel};

const LISTENING_PORT: &str = "10000";

use std::thread;
use std::sync::mpsc;

fn register_blockchain_and_init(engine:&mut Engine)
{
    let chain: Arc<Mutex<Vec<Block>>> = Arc::new(Mutex::new(Vec::new()));
    let mut peers: Vec<String> = Vec::new();

    let listener_chain = chain.clone();
    spawn(|| handle_incoming_connections(listener_chain));

    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);
    let tx3 = mpsc::Sender::clone(&tx1);
    let tx4 = mpsc::Sender::clone(&tx1);

    let add_block = move |data:i32| {
        let cmd = format!("add_block {}",data);
        tx1.send(cmd).unwrap();
    };
    engine.register_fn("add_block", add_block);

    let list_block = move || {
        tx2.send("list_block".to_owned()).unwrap();
    };
    engine.register_fn("list_block", list_block);

    let add_peer = move |peer:String| {
        let cmd = format!("add_peer {}",peer);
        tx3.send("add_peer".to_owned()).unwrap();
    };
    engine.register_fn("add_peer", add_peer);

    let main_loop = move|| {
        loop{
            let input = rx.recv().unwrap();
            
            let splitted: Vec<&str> = input.split(' ').collect();

            /* get() returns &&str, so we mention result type &str
            and get it from a reference (*) */
            let command: &str = match splitted.get(0) {
                Some(value) => *value,
                None => {
                    continue;
                }
            };

            const ADD_BLOCK: &str = "add_block";
            const SEE_BLOCKCHAIN: &str = "list_blocks";
            const ADD_PEER: &str = "add_peer";
            const LIST_PEERS: &str = "list_peers";
            const EXIT: &str = "exit";
            const HELP: &str = "help";

            let option = match splitted.get(1) {
                Some(option) => option,
                None => {
                    if command == ADD_BLOCK || command == ADD_PEER {
                        continue;
                    }

                    ""
                }
            };

            if command == ADD_BLOCK {
                let data: i32 = option.parse().unwrap();
                let mut chain = chain.lock().unwrap();

                let mut previous_digest = String::new();

                if !chain.is_empty() {
                    previous_digest = chain.last().unwrap().get_current().to_string();
                }

                let block = Block::new(data, previous_digest);
                chain.push(block.clone());

                println!("New block added.");

                broadcast_block(&peers, block);
            } else if command == SEE_BLOCKCHAIN {
                list_blocks(&chain);
            } else if command == ADD_PEER {
                let full_address = format!("{}:{}", option, LISTENING_PORT);
                peers.push(full_address.clone());

                println!("Address {} added to peers list.", option);

                let stream = create_stream(&full_address);
                if stream.is_some() {
                    let remote_chain = get_chain_from_stream(stream.unwrap());

                    let mut chain = chain.lock().unwrap();

                    if remote_chain.len() > chain.len() {
                        *chain = remote_chain.clone();
                        println!("The local chain is outdated compared to the remote one, replaced.");
                    } else {
                        println!("The local chain is up-to-date compared to the remote one.");
                    }
                }
            } else if command == LIST_PEERS {
                list_peers(&peers);
            } else if command == HELP {
                list_commands();
            } else if command == EXIT {
                break;
            } else {
                println!("Command not found. Type 'help' to list commands.");
            }
        }
    };
    spawn(main_loop);


}