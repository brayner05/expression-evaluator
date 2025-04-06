# 💻 🧮 = Expression Parser (`pxpr`) = 🧮 💻
PXPR is a fast mathematical expression parser written in Rust. I made this project as a way to familiarize myself with the Rust language, and its features. PXPR is essentially a command-line calculator which works by first tokenizing the input, and then recursively parsing the tokens into an abstract syntax tree which is then evaluated by the program.

## Examples
### Compute an expression via the CLI
```sh
pxpr 3 - (2 + 5) + 2
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
git clone https://github.com/brayner05/expression-evaluator.git
```

Next, switch to the repository directory and run the installation script:
```sh
cd expression-evaluator
chmod +x ./install.sh
./install.sh
```

If no errors occurred, then PXPR is now installed on your machine.