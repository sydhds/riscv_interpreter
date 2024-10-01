# RISC-V interpreter

## Documentation

* ELF file format: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
* RV32I (instruction set): https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf page 116

# Compile code

## Setup

* sudo apt install gcc-riscv64-linux-gnu

## Compile

riscv64-unknown-elf-as -o basic.o basic.s
riscv64-linux-gnu-ld -o basic basic.o
riscv64-unknown-elf-objdump -d ./basic

# Run

* cargo run

# Status

* read elf file format [DONE]
* decode instructions [DONE]
* process instructions [WIP]