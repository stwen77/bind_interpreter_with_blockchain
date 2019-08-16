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
//use rust_blockchain::peers::{create_stream, get_chain_from_stream, list_peers};
//use rust_blockchain::help::list_commands;
//use rust_blockchain::display::{clear_screen, get_input, set_cursor_into_input, set_cursor_into_logs};
//use rust_blockchain::message::{Message, MessageLabel};

const LISTENING_PORT: &str = "10000";

use std::thread;
use std::sync::mpsc;


fn register_blockchain_and_init(engine:&mut Engine)
{
    let chain: Arc<Mutex<Vec<Block>>> = Arc::new(Mutex::new(Vec::new()));
    let mut peers: Vec<String> = Vec::new();

    let listener_chain = chain.clone();
    spawn(|| handle_incoming_connections(listener_chain));


    let add_block = |data:i32| {
        post_cmd_to_main_loop("add_block");
    };

    engine.register_fn("list_block", add_block);

    let list_block = |data:i32| {
        post_cmd_to_main_loop("list_block");
    };

    engine.register_fn("add_block", add_block);
}
pub fn post_cmd_to_main_loop(cmd:&str) {
    println!("{}", cmd);
}