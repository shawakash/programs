# Learning Solana - Basic Program

This is repo is used as an impl of Solana program, built as a learning exercise to understand fundamental concepts of Solana development. The program implements a basic calculator that performs arithmetic operations on stored values.

## What I Learned

- Basic Solana program structure
- Account management
- Instruction handling
- Borsh serialization/deserialization
- Error handling in Solana
- Program architecture and organization

## Program Features

### Basic Operations
- Increment a value
- Decrement a value
- Multiply a value
- Divide a value (with safety checks)

### Project Structure
```
src/
├── instructions/     # Where the actual operations happen
│   ├── calc.rs      # Calculator operations
│   └── mod.rs       # Instruction router
├── lib.rs           # Program entrypoint
├── processor.rs     # Main instruction processor
└── state.rs         # Data structures and state
```

### Key Components

1. **State Management**
   - Uses Borsh for serialization
   - Simple account data structure
   - Instruction type enums

2. **Instructions**
   - Structured in two levels for better organization
   - Basic arithmetic operations
   - Safety checks (e.g., division by zero)

3. **Error Handling**
   - Program ownership validation
   - Operation safety checks
   - Proper error returns

## Notes to Self

- Remember to check account ownership
- Always handle potential arithmetic errors
- Use proper error types
- Keep instruction handling organized

## Future Learning Goals

- [ ] Add tests
- [ ] Implement more complex operations
- [ ] Learn about cross-program invocation
- [ ] Add multiple account handling
- [ ] Explore PDA (Program Derived Addresses)

## Resources Used

- Solana Documentation
- Borsh Documentation
- [Program Examples](https://github.com/solana-developers/program-examples)

## Setup and Build

```bash
# Build
cargo build-bpf

# Deploy to localnet
solana program deploy target/deploy/first_sol.so
```

## Prerequisites

- Rust
- Solana Tool Suite
- Basic understanding of Rust and Solana concepts

---

*This is a learning project and not intended for production use.*
