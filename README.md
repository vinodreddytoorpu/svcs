
# SVCS – SystemVerilog Compiler and Simulator

**SVCS is a modular, Rust-based compiler and simulator for SystemVerilog. It provides a full toolchain from preprocessing to semantic analysis, with a modern CST (Concrete Syntax Tree) and advanced language support.**

![Compilation Flow](docs/compilation.png)

## Key Features

### Modular Architecture
- Each compilation stage is a separate crate:
  - **svcs-preprocessor**: Handles `define`, `include`, and preprocessing directives.
  - **svcs-lexer**: Lexical analysis with robust SystemVerilog keyword/token support.
  - **svcs-parser**: Builds a detailed CST, grouping ports, assignments, module instances, and more.
  - **svcs-analyzer**: Semantic checks and analysis.
  - **svcs-logger**: Advanced logging for all stages.

### Language Support
- Supports SystemVerilog modules, ports, assignments, module instantiations, and many keywords.
- CST groups:
  - Ports (input/output/inout) as `Port` nodes under `PortList`.
  - Assignments as `AssignStatement` nodes.
  - Module instantiations as `Instance` nodes.
  - All tokens include type, lexeme, line, and column.
- Easily extensible for always blocks, parameter lists, and more.

### Command-Line Interface
- Multiple input files: `-i file1.sv file2.sv ...`
- Directory processing: `--dir src/`
- Custom log directory: `--log-dir logs/`
- Log level control: `--log-level debug|info|warn|error`

### Logging & Workspace
- Timestamped log files and always-current `latest.log`
- Dual console and file output with rich formatting
- Cargo workspace for easy dependency and crate management

## Quick Start

```sh
git clone https://github.com/vinodreddytoorpu/svcs.git
cd svcs
cargo build

# Run SVCS on one or more SystemVerilog files
cargo run --bin svcs -- -i systemverilog/examples/full_adder.sv

# Process all .sv files in a directory
cargo run --bin svcs -- --dir systemverilog/examples/
```

## Example

See `systemverilog/examples/full_adder.sv` for a full adder using two half adders. The CST output groups ports, assignments, and instances for easy analysis.

cargo build

# Run SVCS on one or more SystemVerilog files
cargo run --bin svcs -- -i my_design.sv

# Process all .sv files in a directory
cargo run --bin svcs -- --dir src/

# Customize logging
cargo run --bin svcs -- -i test.sv --log-dir build_logs --log-level debug

```

