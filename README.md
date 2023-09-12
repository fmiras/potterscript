# 🧙 PotterScript: The Wizarding World Programming Language
Esoteric Parody Programming Language based in Harry Potter (Wizarding World). Experience the magical world of coding like never before!

## ✨ Introduction
Ever wished you could speak the magical language of wizards and witches? PotterScript is your enchanted gateway to code with the charm and mystery of the Wizarding World.

This project is a remarkable endeavor to understand the magical connection between human logic and machine interpretation. It's a journey filled with trials, failures, and the sweet taste of success that only comes from the relentless pursuit of something extraordinary.

Powered by Rust and its incredible features like godlike error handling and orgasmic memory safety, PotterScript provides a solid toolkit for creating an enchanting programming experience.

## 🚀 Get Started
Ready to embark on this magical journey? There are a few options
- [Web Playground](https://potterscript.fmiras.com) is an interactive playground where you can write, parse and run PotterScript code within the browser. This is possible because PotterScript parser and runtime were adapted and compiled to a [WebAssembly library](wasm).
- [Ron](ron) is a REPL program, basically a terminal-based live console for running PotterScript code.
- You can directly use the [parser](parser) and [runtime](runtime) on your Rust code and do stuff, you can check the [example project](example) that takes an `input.potter` file, parses and evals the code.

## 🧙‍♂️ Features
### Spell Functions
Cast spells with your virtual wand and experience these magical functions:

- **Memory Management:** Use `~Obliviate` to clear memory.
- **System Calls:** Execute `~AvadaKedabra` to exit processes.
- **Magical Expressions:** Perform magical operations like `~Engorgio {integer}` to increment numbers.
- **Visual Spells:** Create visual effects like `~Lumos` to print with black letters on a white background.
- **Emotional Spells:** Feel the power of `~Serpensortia` that returns "🐍", and many more!
A detailed list of spell functions can be found [here](parser/src/lib.rs#L136).

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

### Sorting Hat
Generate `HogwartsHouse` values (is a type like String or Integer), using the Sorting Hat, just like:

```potter
house = 🎩✨ # or SortingHat

if house = Griffindor {
  ~Periculum
}
```

## 🧰 Technology
PotterScript utilizes Rust's excellent [nom](https://github.com/rust-bakery/nom) library to create custom parsers.

Run the following to eval `input.potter` file and use the REPL program:
```bash
cargo run
```

## 📜 License
Licensed under the MIT License. See [LICENSE](LICENSE.md) for more details.

## 🌟 Contribute
PotterScript is open for contributions! Cast your magical spells and bring your creativity to life.

