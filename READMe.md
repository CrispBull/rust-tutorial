## Rust for Kotlin Developers

### Installation

Install Rust following instruction [here](https://www.rust-lang.org/tools/install). For writing Rust, I just
installed intellij RustRover, which has been a really smooth experience so far for me and is free. To creat a Rust
project from terminal after installing Rust you can use the command `cargo new projectName`. This generates a project
with git as well as the base rust file. All rust projects start with from the `main.rs` file. Like Kotlin, Rust
functions don't have to be in a class and can just be functions in a file.

#### Cargo

In the newly created rust project, you'll find the cargo.toml file, Cargo is Rust's version of gradle or maven, so
here where you have your project configurations and settings, dependencies, etc. You can see possible rust libraries
(called crates) on [cargo package registry](https://crates.io/) which you can add to your project. You can read more
about Cargo [here](https://doc.rust-lang.org/cargo/)

### Building and Tooling

- Cargo - (Gradle/Maven)
- Clippy - Kotlin lint
- Rustfmt - Detekt
- Testing - Cargo test

### Basic language constructs

- #### Comments:
  Comments in Rust are similar to comments in Kotlin. Single line comments are written similarly with two forward
  slashes or `/* ..comments.. */`
  ```rust
  // Singe line comment 
  // Another single line comment
  
  /*
  This is another single line comment
  */
   ```
  Documentation comments slightly differ between Kotlin and Rust. In rust, doc comments can start with three forward
  slashes or similar to Kotlin which is `/** ..comments.. **/`. For example;
  ```rust
  /// This function adds two numbers
  /// and returns the result of both numbers
  /// # Example
  /// ```
  /// let sum = add(5, 4);
  /// println!("{}", sum);
  pub fn add(a: i32, b: i32) -> i32 {
      a + b;
  }
  
  /**
  Adds two numbers together and returns the result
  # Arguments
  * `a` - First number to add
  * `b` - Second number to add
  # Returns
  The sum of a and b
  */
  pub fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```
- #### Variables and Mutability:
  Rust is statically typed but can be written dynamically, this is similar to how it is in Kotlin. 
- #### Null safety:
  In Rust, there are no null pointers or references as seen in languages like Kotlin/Java. Instead Rust provides 
  enums like `Option` and `Result` that lets developers handle values or errors explicitly. If there is 
  an error by a function which isn't handled appropriately, Rust can invoke a `panic!()` which would cause the 
  program to terminate with an error message.
- #### Data types
- #### Functions
- #### Control flow
- #### Structs & Enums
- #### Generics & Traits

### Similarities & Differences

- #### Ownership & Borrowing
- #### Memory Management
- #### Lifetimes
- #### Pattern Matching
- #### Error Handling
- #### Modules and Crates
- #### Macros
- #### Concurrency
- #### Async/Await
- #### FFI
- #### Unsafe rust

### Advanced

- #### Kotlin from Rust
- #### Rust from Kotlin

### Sample programs

### Frameworks

### Resources
- [Rust lang book](https://rust-book.cs.brown.edu/) (with quizzes)
- Rust in Action
- Data Structures and Algorithms in Rust
- 

### Notes

- Rust enforces serious compile time checks which catches a lot more when it compiles than a java program would normally
  do. For example, if I have a Rust file with a function that potentially causes a dangling
  pointer error or a racing data error which could potentially cause concurrent modification error, this would make the
  program fail to compile successfully whether those functions are used or not used.
- Unlike in OOP languages like Java where code is organized in classes, Rust does it like Kotlin where code to be 
  executed doesn't have to be in classes and can be in files, which makes us rethink how we manage state. Rust has 
  some OOP features like data encapsulation and polymorphism, doesn't necessarily support it like traditional OOP 
  languages. You can learn more about [OOP in rust here](https://doc.rust-lang.org/book/ch17-00-oop.html)
- Rust does not use a garbage collector to manage memory

### Cons in Rust
- Slower compiler compared to Java/Kotlin due to its extensive checks and optimizations with incremental compilation.
  However, compiling in Rust is generally faster than Kotlin Native. 
- Very strict compiling. This is both a blessing and a curse as this strictness can get in the way and must all be 
  resolved before our program would even compile
- Rust is a large language with several novel concepts like ownership and borrowing, and so it has a steeper 
  learning curve compared to languages like Java/Kotlin
