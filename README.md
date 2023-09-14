# 🧙 PotterScript: The Wizarding World Programming Language

Parody Programming Language based in Harry Potter (Wizarding World). Experience the magical world of coding like never before! It's a really useful to project to learn Rust and how programming languages are done.

## ✨ Introduction

Ever wished you could speak the magical language of wizards and witches? PotterScript is your enchanted gateway to code with the charm and mystery of the Wizarding World.

This project is a remarkable endeavor to understand the magical connection between human logic and machine interpretation. It's a journey filled with trials, failures, and the sweet taste of success that only comes from the relentless pursuit of something extraordinary.

Powered by Rust and its incredible features like godlike error handling and orgasmic memory safety, PotterScript provides a solid toolkit for creating an enchanting programming experience.

## 🚀 Get Started

Ready to embark on this magical journey? Here you have a few options:

- [Web Playground](https://potterscript.fmiras.com) is an interactive playground where you can write, parse and run PotterScript code within the browser. This is possible because PotterScript parser and runtime were adapted and compiled to a [WebAssembly library](wasm).
- [Ron](ron) is a REPL program, basically a terminal-based live console for running PotterScript code.
- You can directly use the [parser](parser) and [runtime](runtime) on your Rust code and do stuff, you can check the [example project](example) that takes an `input.potter` file, parses and evals the code.

## 🧙‍♂️ Features

### Spell Functions

In PotterScript there are no such things as functions or args, but there are **spells** and **targets**. To cast a spell you have to use the wand character `~` following by the spell name and the target expression.

You can only cast a spell on one target, and the spell will do something with the target. For example, the `~Revelio {target}` spell will print the target expression (variable or raw value) to the console.

```potter
~Revelio "Hello World!"
```

Spells can also of course mutate the target, like `~Engorgio {target}` that will increment the target value by one.

List of spells:

- `~Aguamenti` (no target): Returns "💦"
- `~AvadaKedabra` (no target): Process exit (Rust's `panic!()`)
- `~Engorgio {number|string}`: Increments target value by one if number or transforms target string to uppercase
- `~Incendio {string}`: Append "🔥" to target value string
- `~Inmobolus {ms}`: Sleeps for `ms` milliseconds. If running on the browser it will use `setTimeout` and if running on the terminal it will use `std::thread::sleep`
- `~Lumos` (no target): Flags the runtime to start printing values with white background and black text. It doesn't work in WASM (yet) because it's not possible to change the console colors.
- `~Nox` (no target): Flags the runtime to stop printing values with white background and black text
- `~Obliviate {variable}`: Deletes the variable from the runtime memory
- `~OculusReparo` (no target): Returns "👓"
- `~Periculum` (no target): Prints "🔥🔥🔥🔥🔥🔥🔥🔥🔥" to the console
- `~Reducio {number|string}`: Decrements target value by one if number or transforms target string to lowercase
- `~PetrificusTotalus {variable}`: Freezes the variable value, so it can't be mutated anymore (make it const-like)
- `~Revelio {target}`: Prints the target expression to the console
- `~Serpensortia` (no target): Returns "🐍"
- `~WingardiumLeviosa {string}`: Appends new line ("\n") to target string

### Sorting Hat

Generate `HogwartsHouse` values (is a type like String or Integer), using the Sorting Hat, just like:

```potter
house = 🎩✨ # or SortingHat

if house = Griffindor {
  ~Periculum
}
```

### Quidditch Loops

Model your loops after a Quidditch match and use a 'Snitch' condition to end them.

```potter
index = 0

quidditch {
  snake = ~Serpensortia
  ~WingardiumLeviosa snake
  ~WingardiumLeviosa snake
  snake = snake + " some string"
  ~Revelio snake
  ~Incendio snake
  ~Revelio snake
  ~Engorgio index

  if index == 4 {
    snitch # Break loop
  }
}
```

Outputs:

```bash
🐍

 some string
🐍

 some string🔥
🐍

 some string
🐍

 some string🔥
🐍

 some string
🐍

 some string🔥
🐍

 some string
🐍
```

## TODO

### Language

- [x] Spell Casts
- [x] Binary Operations
- [x] Variables
- [x] If Statements
- [x] Quidditch Loops
- [x] Sorting Hat
- [x] Comments
- [ ] Spell definitions

### Packages

- [x] Parser
  - [x] Atoms
  - [x] Expressions
  - [x] Statements
  - [x] Program
- [x] Example
- [x] Runtime
- [ ] REPL
  - [x] Runtime
  - [ ] JIT compiler
- [x] WASM
  - [x] Parser
  - [x] Runtime
- [ ] Playground
  - [x] Parser
  - [x] Runtime
- [ ] Compiler
  - [ ] Binary ([LLVM](https://llvm.org))
  - [ ] WASM

## 📜 License

Licensed under the MIT License. See [LICENSE](LICENSE.md) for more details.

## 🌟 Contribute

PotterScript is open for contributions! Cast your magical spells and bring your creativity to life.

Disclaimer: This project is not affiliated with Warner Bros. Entertainment Inc. or J.K. Rowling in any way.
