# Rust Compiler

This is a compiler for the Teeny Tiny language, implemented in Rust. It compiles `.teeny` files into C, and then uses GCC to compile the C code into executables.

## Prerequisites

- Rust and Cargo ([Install Rust](https://www.rust-lang.org/tools/install))
- GCC (GNU Compiler Collection)

## Installation

1. Clone this repository:
    ```bash
    git clone https://github.com/yourusername/rust-compiler.git
    ```
2. Navigate to the project directory:
    ```bash
    cd rust-compiler
    ```

3. Build the project:
    ```bash
    cargo build --release
    ```

4. Make sure the `compile.sh` script is executable:
    ```bash
    chmod +x compile.sh
    ```

## Usage

1. Write your code and save the  file with a `.teeny` extension.

2. Compile your `.teeny` file:
    ```bash
     bash compile.sh your_file.teeny
    ```
    This will create an executable with the same name as your `.teeny` file.

3. Run the compiled program:
    ```bash
    ./your_file
    ```

## Example

If you have a file named `lol.teeny`:

1. Compile it:
    ```bash
    bash compile.sh lol.teeny
    ```

2. Run it:
    ```
    ./lol .
    ```


