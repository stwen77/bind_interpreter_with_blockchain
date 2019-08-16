//! Handle display routines.

use std::io::stdin;


/// Clear the whole terminal content and generate the default content (bars and titles). Refactored as used multiple times.
pub fn clear_screen() {
    println!("");
}

/// Handles user input and returns that input as a string.
///
/// Returns:
///
/// user input as string
pub fn get_input() -> String {
    set_cursor_into_input();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("cannot read input");

    clear_screen();

    set_cursor_into_logs();

    input.trim().to_string()
}

/// Set the cursor position at the logs area.
pub fn set_cursor_into_logs() {
    //println!("{}", Goto(0, 2));
}

/// Set the cursor position at the input area.
pub fn set_cursor_into_input() {
    //println!("{}", Goto(0, get_terminal_height() - 2));
}
