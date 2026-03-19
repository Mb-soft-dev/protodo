# AGENT.md — protodo

## What is protodo
Lightweight CLI task manager built in Rust with SQLite persistence.
Single binary, no runtime dependencies, embedded database.

---

## Identity & Expertise

Senior Rust developer specializing in CLI applications.
Priorities: performance, correctness, ergonomics — in that order.

### Core competencies
- CLI parsing: `clap`
- Database: `rusqlite` (SQLite embedded)
- Error handling: `thiserror` / `anyhow`
- Testing: 100% coverage enforced, integration tests against real DB
- Modular design, zero unnecessary allocations

---

## Architecture

src/
├── main.rs       # Entrypoint — parses CLI and dispatches to handlers
├── cli.rs        # clap command definitions and argument structs
├── db.rs         # All SQLite interactions (CRUD, pagination queries)
├── task.rs       # Task struct, Status enum, domain logic
└── lib.rs        # Re-exports for integration tests

tests/
└── db_tests.rs   # Integration tests against real SQLite (in-memory or file)

### Module responsibilities
- `cli.rs` → shape of commands only, no business logic
- `db.rs` → SQL only, no formatting or display logic
- `task.rs` → domain types (`Task`, `Status`), no I/O
- `main.rs` → wires cli + db + task, handles output and errors

### Strict rules
- Never put SQL in `main.rs` or `cli.rs`
- Never put display/print logic in `db.rs`
- All DB errors bubble up via `Result`, never panicked

---

## Domain

### Task
```rust
struct Task {
  id: u32,
  description: String,
  status: Status,
}

enum Status { Pending, InProgress, Done }
```

### Commands
| Command          | Args                          | Notes                              |
|------------------|-------------------------------|------------------------------------|
| `add`            | `"description"`               |                                    |
| `list`           | `--page` `--page-size`        | default: page=1, size=10           |
| `update-status`  | `<id>` `<status>`             | status: Pending, InProgress, Done  |
| `delete`         | `<id>`                        |                                    |
| `get-by-id`      | `<id>`                        |                                    |
| `clear-all-tasks`|                               |                                    |

### Pagination rules
- Valid `--page-size` values: `5 10 20 30 40 50`
- Invalid size → hard error, not a warning
- Empty page → display: `"No tasks found for this page."`

---

## Testing

- Test file: `tests/db_tests.rs`
- Tests run against real SQLite (in-memory recommended for isolation)
- 100% coverage enforced — every command has at least one test
- Each test sets up its own DB state, no shared mutable state between tests
```bash
cargo test
```

---

## Build & Run
```bash
cargo build --release
cargo run -- add "my task"
cargo run -- list --page=1 --page-size=10
cargo run -- update-status 1 Done
cargo run -- delete 1
cargo run -- get-by-id 1
cargo run -- clear-all-tasks
```

---

## Future
- Filter by status
- Sort by creation date
- Extended error handling
