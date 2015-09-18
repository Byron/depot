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

## try! for Iterator<Item=Result<T, E>> totally works
* This is valid: `let bytes = try!(try!(File::open("file.txt")).bytes())`
* It's the same as:
```Rust
let bytes = Vec::new();
for byte_result in try!(File::open("file.txt")).bytes() {
  match byte_result {
    Ok(byte) => bytes.push(byte),
    Err(err) => panic!("{}", err),
  }
}
```

## Google.rs - Dev Diary

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

### Episode 2: One CLI per API

* how to ? impossible to do without code generation.
    - pre-CLI: d1c5bf1
* documentation: Which documentation system to use ? Mkdocs to the rescue (markdown to html with themes). Python preferred
    - show online
* build system: show how to extend build system to build any program type with minimal redundancy
    - mkdoc generator works, CLI is included in build system: d1c9791
* Argument Parsing: let's try to not deal with it, for now. -> docopts
    - basic usage: 390354b
* request structures: how to set request structures: use a cursor-based approach
    - show structure in code/final docs
    - explain cursor
* compilation: if debug-compilation is pain, release is ... hell
    - talk about it, maybe on camera

.. last words, on camera

### Episode 3: To make it work right !

[pixellated screen] tell in camera that the youtube3 related tools improved workflow leading to better bandwidth
utilitzation and improved quality [depixellate screen]. Tout improved sound and higher production values

[Cut in Logo Animation and start voice-over in trailer-like quality] - Welcome to another episode of the Google.rs
developer diary.

"To make it work right"

#### Rocking Blockers I
"I don't know what the title wants to imply ! Even the release v0.1.0 is working alright, let me show you !!"
[show screenshots/demos of problems]
* Https Uploads are cached in memory ... " ... but this isn't a problem for files smaller than 2GB ... I can still do it "
* Null values in meta-data are rejected by server "... DUH ! Damnit! "

* [Voice in Background] "This made Byron very sad, seeing his work shattered like that ..."
* [Voice] "... then angry" : "Memory Bug: You gonna be crushed"
* Show how it's tracked down through hyper to `rust-openssl`. Show how issue is placed.
* "Alright, next one: Null values !" [Show Byron thinking ... brabbling to finally ask for help on github]
* "Nothing left to do here ... let's distract myself with ... usability" [on last word, faces camera, breaks third wall ;)]

#### It's all about usability

* first version wasn't quite there, with an argparser who couldn't tell more than just: syntax error. Show how difficult
  usage is.
* "I need something better ... " [slow affirmative clapping] " ... but what could it be ?!"
* [slow cut to clap-rs github page]: "Right ... :)"
* Show how appealing the project is thanks to its downright gorgeous documentation, and how nice the output looks.
* **Step 1**: Naïve Implementation
  * Show stack-based version, and stackoverflow
  * show that there is no way to control the size of the main thread in Rust
* **Step 2**: Put workload to compile-time
  * Show what the problem (probably) is: App Structures requiring stack space
  * Show how the data can be restructured to be more lightweight.
  * Setup App at runtime
* **Step 3**: 'Did you mean' and finishing touches
  * Show that 'required parameter' information wasn't there
  * Show using 'git' that Did-you-mean would be great
  * Show how it is implemented
    - for request values/paramters
    - for clap-rs in general (subcommands, long flags)

#### Rocking Blockers II

It's still not working, let's see what happened in the meanwhile

* Show that openssl issue was fixed ! Yeah
* However, no reply from serde.
  * "Erick, Whyyyyyy ?!?" [hold camera, dramatic why, break down to the ground, black out camera]
  * [Fade in camera] "Ok, so I gotta do it myself then !!!"
* **json-tools**
  * "What would the simplest solution be ... hmm ... sure, a lexer, lexical-filter and serializer for json"
  * "Sounds like a great Idea, let's do it !" [show fingers going down on keyboard, pretending touch-typing, but doing eagle style]
  * show in fast-forward style which parts are needed to do lexical level filtering
  * ... and show final results
* Now that it's done, show Erick's reply and how simple it could have been
  * "Just a few days late ;) ... but hey, I will use both ways !"
  * Show how json tools are used in the API, and the json::Value style in the CLI !

TODO: Wrap it up, last words

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
