# 💻 🧮 = PXPR - Expression Parser = 🧮 💻
PXPR (**P**arse e**XPR**ession) is a fast arithmetic and boolean expression parser written in Rust, supporting arithmetic operations such as addition, subtraction, multiplication, division, modulo, 
expressions with nested parentheses, as well as C-style boolean expressions such as the logical not, and, or operators and the logical implication operator. PXPR can be evaluate expressions directly from the command line
or via the PXPR REPL.

## Sections
- [The goal of PXPR](#what-is-the-goal-of-pxpr]
- [How to use PXPR](#how-to-use-pxpr)
- [Installing PXPR](#Installation)

## What is the goal of PXPR?
The goal of PXPR is to provide an easy-to-use, but feature rich command line calculator. Rather than opening up an IDE or text-editor, writing some code, compiling that code, and running it all to get the value 
of a basic expression, PXPR can evaluate the expression directly from the command line.


## How to use PXPR
### Compute an expression via the CLI
```sh
pxpr "3 - (2 + 5) + 2"
```
Which outputs:
```sh
    = -2
```

### Compute an expression via the REPL
```sh
pxpr
```
Which will initialize the REPL:
```
expr > 3 - (2 + 5) + 2
        = -2
```

## Installation
To install PXPR, clone this repository:
```sh
git clone https://github.com/brayner05/pxpr.git
```

### Installing PXPR on
- [Linux](###Linux)
- [Windows](###Windows)

### Linux
Once the PXPR repository has been cloned, switch to the repository directory and run the installation script:
```sh
cd pxpr
chmod +x ./install.sh
./install.sh
```

If no errors occurred, then PXPR is now installed on your machine.


### Windows
Currently PXPR does not have an installation script for Windows, but can still be installed quite easily:
```batch
cd pxpr
cargo build --release
cargo install
```
