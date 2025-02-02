use std::io::{self, Write};
#[cfg(test)]
use tbg::terminal_utils::clear_console;
use termion::{clear, cursor};

// Test clearing the console
//
// This is a little weird because we can't really test the actual terminal state.
// We're just relying on termion to print the `expected_clear_sequence`, but
// that doesn't actually "remove" anything from stdout. Stdout is 4ever.
//
// In other words we can't really check that the test message is cleared, i.e.,
// assert!(
//     !result.contains("Test message"),
//     "Expected the message to be cleared."
// );
//
// So instead, we just check that the `expected_clear_sequence` is there.
#[test]
fn test_clear_console() {
    let mut output = Vec::new(); // Collect the output in this buffer
    let mut stdout = io::Cursor::new(&mut output); // Redirect stdout

    // Write a test message to simulate something printed to the console
    write!(stdout, "Test message").unwrap();

    // Call the clear_console function to clear the terminal
    clear_console(Some(&mut stdout));

    // Check that escape sequences are written and the message is removed
    let expected_clear_sequence = format!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);
    let result = String::from_utf8_lossy(&output).to_string();

    // Assert that clear sequence is part of the output
    assert!(
        result.contains(&expected_clear_sequence),
        "Expected output to contain: {}",
        expected_clear_sequence
    );
}
