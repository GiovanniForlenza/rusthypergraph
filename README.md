# RustHypergraph (RHG)

RustHypergraph (RHG) is a Python library written in Rust that provides functionality for working with hypergraphs.

## Features

- TODO

## How to Install

### For Linux and MacOS

Before you begin, make sure you have Git, Python, and Rust installed on your machine.

As the library is not yet available on PyPI, the steps to install and use it are as follows:

1. Clone the library:
    ```
    git clone https://github.com/GiovanniForlenza/rusthypergraph.git
    ```

2. Navigate to the `rusthypergraph` directory:
    ```
    cd rusthypergraph
    ```

3. Build the library:
    ```
    cargo build --release
    ```

4. Now you need to install Maturin:
- It's advisable to use a virtual environment:
  ```
  python3 -m venv .env
  ```
- Activate the virtual environment:
  ```
  source .env/bin/activate
  ```

5. To install Maturin, execute:
    ```
    pip install maturin
    ```

6. To compile the Rust extension for the Python project:
    ```
    maturin develop
    ```

7. To install the library in the virtual environment, simply execute:
    ```
    pip install .
    ```

8. If you want to install the library on your machine and use it outside the virtual environment:
    ```
    deactive
    ```
    
    ```
    pip install .
    ```
Perfect, now the library is installed and ready for use 🔥