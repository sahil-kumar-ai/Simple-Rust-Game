# 🦀 Tic-Tac-Toe (Rust)

A command-line Tic-Tac-Toe game built with **Rust**, featuring two-player gameplay, interactive menus, score tracking, and win detection.

This project was built to practice Rust fundamentals, including structs, enums, ownership, borrowing, arrays, iterators, and modules.

## 🎮 Features

- **Two-Player Gameplay** – Play with X and O, taking alternate turns.
- **Interactive CLI Menu** – Navigate game options using arrow keys and Enter.
- **Input Validation** – Validate row and column inputs (1–3).
- **Move Validation** – Prevent players from selecting an already occupied position.
- **Win Detection** – Detect all 8 winning combinations (3 rows, 3 columns, 2 diagonals).
- **Draw Detection** – Detect when the board is full without a winner.
- **Colored Output** – Highlight winning combinations in the terminal.
- **Scoreboard** – Track X wins, O wins, and draws across rounds.
- **Game Reset** – Reset the board while optionally preserving scores.

## 🛠️ Tech Stack

- **Language:** Rust
- **Dependency:** [dialoguer](https://crates.io/crates/dialoguer) – Interactive terminal menus.

## 📁 Project Structure

```text
Tic-Tac-Toe/
│
├── Cargo.toml
├── Cargo.lock
│
└── src/
    ├── main.rs
    ├── board.rs
    ├── moves.rs
    └── intractive.rs
```

| File | Responsibility |
|---|---|
| `main.rs` | Main game loop and game flow |
| `board.rs` | Board management, win detection, draw detection, and scoreboard |
| `moves.rs` | Player input, position handling, and Move enum |
| `intractive.rs` | Interactive CLI menu using dialoguer |

## ⚙️ Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install).
- Cargo comes bundled with Rust.

### Clone the repository

```bash
git clone https://github.com/sahil-kumar-ai/Simple-Rust-Game.git
```

### Navigate to the project

```bash
cd Simple-Rust-Game/Tic-Tac-Toe
```

### Run the game

```bash
cargo run
```

Cargo will automatically download the required dependencies and compile the project.

## 🧠 Rust Concepts Practiced

- Structs and implementations (`impl`)
- Enums and pattern matching
- Ownership and borrowing
- Mutable references
- Arrays and nested arrays
- Iterators (`iter`, `enumerate`, `position`)
- Modules and visibility (`pub`)
- Method chaining
- External crate integration
- CLI input handling

## 🚀 Future Improvements

- Replace boolean player tracking with a `Player` enum.
- Improve error handling using `Result` and `?`.
- Add game saving and loading.
- Implement a computer opponent using the Minimax algorithm.

## 👨‍💻 Author

**Sahil Kumar**

[GitHub](https://github.com/sahil-kumar-ai)

---

Built with Rust 🦀