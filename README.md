# Interpretor in Rust

A simple tree-walk interpreter written in Rust for an imaginary language.

## Table of Contents
- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Language Grammar](#language-grammar)
  - [Syntax](#syntax)
  - [High-Level Characteristics](#high-level-characteristics)
    - [Dynamic Typing](#dynamic-typing)
    - [Automatic Memory Management](#automatic-memory-management)
  - [Data Types](#data-types)
    - [Booleans](#booleans)
    - [Numbers](#numbers)
    - [Strings](#strings)
    - [Nil](#nil)
  - [Expressions](#expressions)
    - [Arithmetic](#arithmetic)
    - [Comparison and Equality](#comparison-and-equality)
    - [Logical Operators](#logical-operators)
    - [Precedence and Grouping](#precedence-and-grouping)
  - [Statements](#statements)
    - [Expression Statements](#expression-statements)
    - [Blocks](#blocks)
  - [Variables](#variables)
    - [Declaration and Assignment](#declaration-and-assignment)
  - [Control Flow](#control-flow)
    - [If Statements](#if-statements)
    - [While Loops](#while-loops)
    - [For Loops](#for-loops)
  - [Functions](#functions)
    - [Function Calls](#function-calls)
    - [Function Definitions](#function-definitions)
    - [Closures](#closures)
  - [Classes](#classes)
    - [Class Declaration](#class-declaration)
    - [Instantiation](#instantiation)
    - [Initialization](#initialization)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Description
This project is a Rust-based interpreter that executes a custom programming language designed for educational purposes and efficient performance.

## Installation
To install and run the interpreter, follow these steps:

1. **Clone the repository:**
    ```sh
    git clone https://github.com/yaegeristhitesh/RLox
    ```
2. **Navigate to the project directory:**
    ```sh
    cd rlox-basic/
    ```
3. **Build the project:**
    ```sh
    cargo build --release
    ```

## Usage
After building the project, you can run the interpreter with the following command:

```sh
./target/release/rlox_basic yourscript.lox
```

## Features
- **Basic arithmetic operations**: Support for addition, subtraction, multiplication, and division.
- **Variable declarations**: Ability to declare and use variables in scripts.
- **Control structures**: Includes if-else statements and while loops for flow control.
- **Functions**: Define and call functions with parameters and return values.
- **Standard library**: A small standard library with useful functions for common tasks.



## Lox Language Features

### 1. Syntax
Lox's syntax is similar to C, using semicolons to terminate statements and allowing single-line comments with `//`. For example:
```lox
print "Hello, world!";
```
Lox avoids static typing, embracing a dynamic typing system to simplify its syntax and reduce complexity.

### 2. High-Level Characteristics

#### 2.1 Dynamic Typing
Lox uses dynamic typing, meaning that variables can hold values of any type and change types during execution. Type errors are detected at runtime. This approach simplifies language implementation and avoids the complexity of static type systems.

#### 2.2 Automatic Memory Management
Lox employs tracing garbage collection (GC) to manage memory automatically. This technique is more robust than reference counting, handling cyclic references and simplifying memory management compared to manual allocation and deallocation.

### 3. Data Types
Lox supports the following built-in data types:

#### 3.1 Booleans
- `true` and `false` are the Boolean values.
- Example:
  ```lox
  true;  // Boolean true
  false; // Boolean false
  ```

#### 3.2 Numbers
- Only double-precision floating-point numbers are supported.
- Example:
  ```lox
  1234;  // Integer representation
  12.34; // Decimal number
  ```

#### 3.3 Strings
- Strings are enclosed in double quotes.
- Example:
  ```lox
  "I am a string";
  ""   // Empty string
  "123" // String containing digits
  ```

#### 3.4 Nil
- Represents a non-value, similar to `null` in other languages.
- Example:
  ```lox
  nil;
  ```

### 4. Expressions

#### 4.1 Arithmetic
- Operators: `+`, `-`, `*`, `/`
- Example:
  ```lox
  3 + 4; // 7
  -5;    // -5
  ```

#### 4.2 Comparison and Equality
- Operators: `<`, `<=`, `>`, `>=`, `==`, `!=`
- Example:
  ```lox
  5 < 10;  // true
  "cat" == "dog"; // false
  ```

#### 4.3 Logical Operators
- `and`, `or`, `!`
- Example:
  ```lox
  true and false; // false
  !true;          // false
  ```

#### 4.4 Precedence and Grouping
- Operators follow typical precedence rules, and parentheses can be used for grouping.
- Example:
  ```lox
  (1 + 2) * 3; // 9
  ```

### 5. Statements

#### 5.1 Expression Statements
- Expressions can be promoted to statements with a trailing semicolon.
- Example:
  ```lox
  print "Hello";
  ```

#### 5.2 Blocks
- Group multiple statements within `{}`.
- Example:
  ```lox
  {
    print "One statement.";
    print "Two statements.";
  }
  ```

### 6. Variables

#### 6.1 Declaration and Assignment
- Use `var` to declare variables. Variables default to `nil` if not initialized.
- Example:
  ```lox
  var x = 10;
  var y;
  y = x;
  ```

### 7. Control Flow

#### 7.1 If Statements
- Executes code based on a condition.
- Example:
  ```lox
  if (x > 0) {
    print "Positive";
  } else {
    print "Non-positive";
  }
  ```

#### 7.2 While Loops
- Repeats code as long as a condition is true.
- Example:
  ```lox
  while (x > 0) {
    print x;
    x = x - 1;
  }
  ```

#### 7.3 For Loops
- Executes code with initialization, condition, and iteration.
- Example:
  ```lox
  for (var i = 0; i < 5; i = i + 1) {
    print i;
  }
  ```

### 8. Functions

#### 8.1 Function Calls
- Functions are called with parentheses, with or without arguments.
- Example:
  ```lox
  sum(1, 2);
  ```

#### 8.2 Function Definitions
- Define functions with `fun`.
- Example:
  ```lox
  fun greet(name) {
    print "Hello, " + name;
  }
  ```

#### 8.3 Closures
- Functions can reference variables from their enclosing scopes.
- Example:
  ```lox
  fun makeCounter() {
    var count = 0;
    fun increment() {
      count = count + 1;
      return count;
    }
    return increment;
  }
  ```

### 9. Classes

#### 9.1 Class Declaration
- Define classes with `class`, including methods.
- Example:
  ```lox
  class Person {
    greet() {
      print "Hello!";
    }
  }
  ```

#### 9.2 Instantiation
- Create instances by calling the class like a function.
- Example:
  ```lox
  var person = Person();
  ```

#### 9.3 Initialization
- Define an `init` method for initialization.
- Example:
  ```lox
  class Person {
    init(name) {
      this.name = name;
    }
    greet() {
      print "Hello, " + this.name;
    }
  }
  ```

## Contributing
We welcome contributions! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature-name`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature-name`).
5. Open a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements
- Inspired by the book "Crafting Interpreters" by Robert Nystrom.
- Thanks to the Rust community for their support and contributions.
