//
use crate::terminal_utils;

pub fn run() {
    // TODO:
    // Load player, get x and y
    // Load map
    //  - How to store the map?
    //
    // Start a world_navigation loop
    //  - When player uses arrow keys, their movement choice is validated against the map
    //  - Can't move into walls, etc.
    //  - When they press x, they inspect whatever they're next to.
    //
    // Design decisions
    //  - How to give player "directionality" in the terminal?
    //  - WIP:
    //      - left - [<o]
    //      - right - [o>]
    //      - up - |o^|
    //      - down - |ov|

    let content = format!("x");
    terminal_utils::draw_window(&content).expect("Failed to draw window in dev");
    terminal_utils::prompt_enter_to_continue();
    // world_navigation_utils::prompt_for_action();
}
