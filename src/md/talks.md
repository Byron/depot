## Rusty Snacks (Short useful vids about Rust)

### Unit Assignments to debug types

`let foo: () = something_complicate()`

### Borrow trait
* How to use Borrow/BorrowMut to become indepent of whether something is borrowed or not in generic code.

### RefCell Usage
* Point out that Rc is by no means required ... explain why Rc is used.

### Expressionistic
* Everything is an expression
* Show how this can be used to assign the result of `match` or `if` to a variable, to prevent deep nesting.

### Type Hinting
* All the different ways of defining types
 * indirectly: let mut v = Vec::new(); v.push(5);
 * definition: let mut v: Vec<u32> = Vec::new();
 * explicitly: let mut v = Vec::new::<u32>();
 * call trait: let mut v = <Vec<u32> as Default>::default();

### Traits and Generics
* implications: traits make types harder to use
* you want traits though to make types easier to use in templates (to hide template parameters)


## Daho.am

### The Google Dialogs - generating REST-APIs for Rust

TODO: details - daho.am pinged to see if there is demand.

## Google-APIs-Rust - Dev Diary

### Episode 1: Code Generation

* authentication is THE prerequisite
* learning from the best !
* it's incredibly boring, error prone and time-consuming to implement a protocol by yourself
* which generator to use ? (Show page about generators) Let's put it to the test
  - gsl (f2ca8c3)
  - pyratemp (c0bfeab)
  - mako (30041e9)
* types can get crazy ! (show type nesting)
* json encoding: field renaming is a requirement ('type' fields and camelCase variable names)
* about docs:
  - `rustdoc` is great and makes awesome documentation possible.
* it's always a gamble to update your compiler or any of your dependencies ([scotty quote](https://www.youtube.com/watch?v=jxzYTDX8bIg))

## Munich Gophers

### Event September 2014

*Just a copy from gophers page, don't know where original notes are*

Learning a new programming language is best achieved by solving an actual problem. godi is the project I used to learn go and I will share the most exciting things about go that were revealed to me in the process. The speech will be held from the perspective of a go-beginner, showing aspects of the go language using actual code of godi.

### Event February 2015

**Ru-Go-La - It's not a salad !**

Rust is the up-and-coming systems programming language that promises safety without sacrificing performance. Go on the other hand already established itself as language for building "[simple, reliable and efficient software](http://golang.org/)".

The first part shall be a short Rust introduction, whereas the second one will show Rust's main language features using actual programs and compares them to the respective go implementation.

Topics we touch will be:

* Rust Intro
 * Safety
  * Borrow-Checker
  * Mutability
  * Error Handling
 * Efficiency/Performance
  * Generics
  * no garbage collection
  * Concurrency
  * Small, static binaries
 * Feature Highlight
  * FFI
  * everything is an expression

* Project Setup and Workflow
 * A new Binary, using a new library
 * Testing (tell about rust doc-tests, and examples)
 * Documentation (show http results, and golang.org)
 * Benchmarking
 * Dependencies (see also [godepdency handling](https://code.google.com/p/go-wiki/wiki/PackageManagementTools))
 * Source-Deployment (tell about local dependency overrides)
 * Cross-Platform Deployment
 
* Primary Language Features
 * Safety
  * Access of Invalid Pointers (The Borrow-Checker)
  * Memory Leaks
  * Race Conditions
  * Starvation/Dead-Lock
  * Error Handling
  * Unicode (All strings are guaranteed to be valid unicode)
 * Performance
  * Compilation
  * Runtime
 * Efficiency
  * Binary Size
  * Runtime Memory Consumption
 * Convenience
  * Overall project setup and deployment (show [cargo manifest information](http://doc.crates.io/manifest.html#the-project-layout))
  * Cross-platform compilation
 * Other Features
  * Polymorphism
  * Concurrency
  * Metaprogramming
  * Reflection
  * C-Interop

Other Differences

Difference      | Rust                      | Go                       |
--------------- | ------------------------- | ------------------------ |
indentation     | 4 spaces                  | 1 tab                    |
vars writable   | on demand only            | yes                      |
names           | underscores               | camelCase                |
doc-comments    | //! and ///               | normal comments          |
expressions are | everything                | some things              |
auto-deref      | method dispatch,recursive | one level                |
& operator      | is borrow operator        | is address-off operator  |
Exceptions      | Panic                     | Panic + Recover          |
Threading-Model | Native Threads (NT)       | Goroutines (on NT)       |

* Is explicit better than implicit ?

#### Notes to self

* cargo builds currently only multi-threaded for dependencies, not for builds itself.
* rustc builds are single-threaded only, as each artifact 'contains' everything else.
* Themes
 * go: bluelover
 * rust: red ( or brunette, but is too dark)

