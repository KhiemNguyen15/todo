# Todo CLI

A simple yet powerful command-line interface (CLI) application for managing your todo lists, built with Rust.

## Installation

### Prebuilt Binary

Download the latest release for your platform from the [Releases](https://github.com/KhiemNguyen15/todo/releases) page and extract it:
```bash
# For Linux (amd64)
wget https://github.com/KhiemNguyen/todo/releases/download/v0.1.0/todo-0.1.0-linux-amd64.tar.gz

# Extract the archive
tar -xzvf todo-0.1.0-linux-amd64.tar.gz

# Move the binary to a location in your PATH
mv todo $HOME/.local/bin/
```

### From Source

#### Requirements

Ensure that you have Rust and Cargo installed on your system.

#### Build Steps

1.  **Clone and enter the repository:**
    ```bash
    git clone https://github.com/KhiemNguyen15/todo.git && cd todo
    ```

2.  **Build the project:**
    ```bash
    cargo build --release
    ```

3.  **Run the application:**
    The executable will be located at `./target/release/todo`. You can run it directly or copy it to a directory in your system's PATH (e.g., `/usr/local/bin` or `~/.local/bin`) for easier access.
    ```bash
    mv ./target/release/todo $HOME/.local/bin/
    ```

## Usage

Here are examples of how to use the `todo` CLI:

**1. Add a new task:**
```bash
todo add "Say hi to mom"
```

**2. List all tasks:**
```bash
todo list
```

**3. Mark a task as done (using its ID):**
```bash
todo done <task_id>
```

**4. Delete a task (using its ID):**
```bash
todo remove <task_id>
```

## Data Storage

Task data is stored locally in a SQLite database file. The exact location is determined using the `directories` crate to find the appropriate user data directory for your operating system (e.g., `$HOME/.local/share/todo/tasks.db` on Linux).

## License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.