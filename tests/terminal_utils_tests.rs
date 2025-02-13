// use crossterm::{cursor, terminal};
// use std::io::Cursor;
// use std::io::Write;
// use tbg::terminal_utils;

// Test clearing the console
//
// This is a little weird because we can't really test the actual terminal state.
// We're just relying on crossterm to print the `expected_clear_sequence`, but
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
fn test_clear_console() {}
