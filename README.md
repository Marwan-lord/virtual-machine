# VM16: 16-Bit Virtual Machine and Assembler

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight 16-bit virtual machine with a custom assembler, designed for learning and  experimentation. Execute custom assembly code on a simulated 16-bit architecture!

---

## Table of Contents
- [Features](#features)
- [Quick Start](#quick-start)
- [Example Programs](#example-programs)
- [Documentation](#documentation)
- [License](#license)

---

## Features
- **16-Bit Architecture**: 4 general-purpose registers. 
- **Instruction Set**: Includes arithmetic, stack operations.
- **Portable**: Written in Rust for cross-platform use.
- **Educational**: Ideal for learning low-level programming.

---

## Quick Start

### Run a Sample Program
```bash
# Clone the repo
git clone https://github.com/Marwan-lord/virtual-machine.git vm16
cd vm16

cargo run --bin asm <assembly>.asm >> <executable>.bin
cargo run --bin vm <executable>.bin
```

## Example Programs
```asm
Push 10
Push 20
AddStack
PopRegister A
Signal $f0
```

## Documentation
[here](https://github.com/Marwan-lord/)
## License 

```

Copyright (c) 2025 Marwan Mohamed

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```
