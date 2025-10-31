# Toy EVM
A minimal Ethereum Virtual Machine (EVM) implementation written in Rust for educational purposes.

## Overview
This project is a simplified implementation of the Ethereum Virtual Machine, designed to help understand how the EVM works internally. It implements core EVM components including memory, stack, opcodes, and basic execution logic.

## Project Structure
The project is organized as a Rust workspace with the following crates:
```
toy_evm/
├── bins/
│   └── evm/              # Main executable binary
├── crates/
│   ├── evm_core/         # Core EVM execution logic
│   ├── primitives/       # Primitive data structures (Memory, Stack)
│   └── env_core/         # Environment-related types
```

## Components
### Primitives (crates/primitives/)
Core data structures used by the EVM:

- Memory (memory.rs) - EVM memory implementation with word and byte operations
- Stack (stack.rs) - EVM stack with push/pop operations
- EVM Types - Block environment, transaction, and storage types

### EVM Core (crates/evm_core/)
The main EVM execution engine:

- Opcodes (opcodes.rs) - Complete set of EVM opcodes including:

  - Arithmetic operations (ADD, MUL, SUB, DIV, etc.)
  - Comparison and bitwise operations (LT, GT, AND, OR, XOR, etc.)
  - Stack operations (PUSH, POP, DUP, SWAP)
  Memory and storage operations (MLOAD, MSTORE, SLOAD, SSTORE)
  - Control flow (JUMP, JUMPI, JUMPDEST)
  - System operations (CALL, CREATE, RETURN, etc.)
  - And many more...
- EVM - Main execution context containing block environment, transaction, memory, stack, storage, and program counter

## Building
```
cargo build
```
## Running
```
cargo run --bin evm
```
## Testing
```
cargo test
```

## Dependencies
- [Alloy](https://github.com/alloy-rs/alloy) - Ethereum types and utilities

## Status
This is a work-in-progress educational project. Not all opcodes are fully implemented yet.

## License
This project is for educational purposes.