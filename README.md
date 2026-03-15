# Protodo CLI Application

The `protodo` CLI tool is a lightweight and functional task management application built with Rust. It provides the ability to manage your tasks directly from the command line.

## Features

- **Task Management**:
  - Add, list, delete, and update task status.
  - Supported statuses: `Pending`, `InProgress`, `Done`.
- **Pagination**:
  - Enforced pagination in the `list` command.
  - Customizable parameters: `--page` (default: 1) and `--page-size` (default: 10).
  - Valid `--page-size` values: `5`, `10`, `20`, `30`, `40`, `50`.
  - Displays descriptive warnings for empty pages.
- **Database Integration**:
  - SQLite backend for persistent task management.
- **Simple, Modular Design**:
  - Organized with clearly separated modules for CLI, database management, and task representation.

---

## Installation

### Requirements
- Rust (1.65.0 or higher)
- SQLite (embedded)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/protodo.git
   cd protodo
   ```
2. Build the application:
   ```bash
   cargo build --release
   ```
3. Run directly:
   ```bash
   cargo run -- <command>
   ```

---

## Usage

### Basic Commands

#### Add a Task
```bash
cargo run -- add "Task description here"
```
#### List Tasks (with Pagination)
```bash
cargo run -- list --page=1 --page-size=10
```
#### Update Status
```bash
cargo run -- update-status <task-id> Done
```
#### Delete a Task
```bash
cargo run -- delete <task-id>
```
#### Clear All Tasks
```bash
cargo run -- clear-all-tasks
```
#### Get a Task by ID
```bash
cargo run -- get-by-id <task-id>
```

---

## Example: Listing Tasks with Pagination

Here’s how to view tasks with pagination:
```bash
cargo run -- list --page=2 --page-size=5
```
- This displays tasks on page 2 with 5 tasks per page.

Empty pages will display:
```bash
No tasks found for this page.
```

---

## Tests

To run the tests:
```bash
cargo test
```
The test suite validates CRUD operations, pagination behavior, and database interactions.

---

## Future Enhancements
- Filter tasks by status.
- Add sorting capabilities (e.g., by creation time).
- Improve error handling for additional edge cases.

---

The `protodo` CLI tool simplifies task management and is a great starting point for Rust learners exploring command-line applications.