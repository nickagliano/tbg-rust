# The Book Game (tbg)

## Overview
**The Book Game (tbg)** is an interactive, text-based, adventure game written in Rust. It features persistent player data stored in an SQLite database, creating an immersive and evolving gameplay experience.

At the core of The Book Game is a turned-based strategy game, where players must skillfully navigate through pages of a book in order to defeat their opponent.

## Features
- Interactive text-based gameplay
- Persistent player profiles stored in an SQLite database
- Console interface with clear prompts and user input handling
- Modular design with a focus on reusability and maintainability

## Installation
### Prerequisites
- Rust (latest stable version recommended)
- SQLite (included via Rust dependencies)

### Clone the Repository
```sh
git clone https://github.com/nickagliano/tbg.git
cd tbg
```

### Build the Project
```sh
cargo build --release
```

### Run the Game
```sh
cargo run
```

## Usage
Upon launching the game, you will be prompted to either continue an existing adventure or create a new character.

- If you are a returning player, your saved data will be loaded.
- If you are a new player, you will be prompted to enter your name before beginning your journey.

## Project Structure
```
tbg/
├── saves/                 # For storing save files (sqlite databases)
├── src/
│   ├── db/                # Database connection and management
│   ├── models/            # Game data structures (e.g., Player, GameState)  
│   ├── args.rs            # For parsing command-line input (mostly for development purposes)
│   ├── lib.rs             # Game logic and core functionality
│   ├── main.rs            # Entry point of the game
│   ├── terminal_utils.rs  # For all things manipulating the terminal
│   ├── test_utils/        # Utilities for testing
├── tests/                 # Tests
├── Cargo.toml             # Rust package configuration
├── README.md              # Project documentation
```

## Dependencies
This project relies on the following Rust crates:
- `termion` – for handling terminal interactions
- `rusqlite` – for SQLite database support

## Contributing
Contributions are welcome! Feel free to fork the repository and submit pull requests.

## License
This project is licensed under the MIT License. See `LICENSE` for details.

## Author
Developed by **Nick Agliano** – [GitHub Profile](https://github.com/nickagliano)
