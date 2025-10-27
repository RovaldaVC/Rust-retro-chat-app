[README-Rust-retro-chat-app.md](https://github.com/user-attachments/files/23163330/README-Rust-retro-chat-app.md)
# Rust Retro Chat App

A simple chat application built with Rust, aiming for a nostalgic, retro feel.

## Key Features & Benefits

*   **Simple Chat Functionality:** Basic text-based communication between multiple clients.
*   **Rust Powered:** Leverages Rust's performance, safety, and concurrency features.
*   **Retro Aesthetic:**  (Future Implementation) Aims to emulate the look and feel of older chat programs.
*   **Client-Server Architecture:** Clear separation of client and server logic.

## Prerequisites & Dependencies

*   **Rust Toolchain:** You need to have Rust and Cargo installed.  Download from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Ensure `cargo`, `rustc`, and `rustup` are properly installed and in your PATH.

## Installation & Setup Instructions

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/RovaldaVC/Rust-retro-chat-app.git
    cd Rust-retro-chat-app
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

## Usage Examples

1.  **Run the Server:**

    ```bash
    ./target/release/server
    ```

2.  **Run the Client:**

    Open a new terminal window for each client you want to run.

    ```bash
    ./target/release/client
    ```

    *   You may need to specify the server address if it's not running on `localhost:1234`.  (Future implementation - currently hardcoded)

## Configuration Options

Currently, the server address and port are hardcoded within the `client.rs` and `server.rs` files.  Future versions will allow configuration through command-line arguments or a configuration file.

## Contributing Guidelines

We welcome contributions to improve this project!

1.  **Fork the repository.**
2.  **Create a new branch for your feature or bug fix.**
3.  **Make your changes and commit them with descriptive commit messages.**
4.  **Submit a pull request with a clear explanation of your changes.**

Please follow these guidelines:

*   Ensure your code adheres to Rust's coding standards.
*   Write clear and concise commit messages.
*   Test your changes thoroughly.
*   Include relevant documentation.

## License Information

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project utilizes the Tokio runtime for asynchronous operations.
