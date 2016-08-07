# Languages

## Crystal

### Pro

* A single binary to do it all: `crystal`
  * build [`--release`]
  * run
  * test publish
  * format
  * introspection (types, context, everything !!!)
* Huge standard library, *batteries included*
  * Binaries are small and only contain what's needed (80kb for a 'hello world')
* Very nice, ruby-like syntax, and generally dedicated to take what Ruby does right
* It's FAST out of the box. Recursive fibonaci beats what comes out of Rust by 60% ... just like that.
* Low memory footprint, generally
* *Fibers* are Crystal's version of a co-routine. A light-weight routine to run asynchronously. *Channels* are used to let them communicate.
* [Crystals shard repository](crystalshards.xyz) is already up and running ! It's like `crates.io`.
* [awesome crystal](https://github.com/veelenga/awesome-crystal) for a curated list of shards.
* 💗 Powerful meta-programming, deeply embedded into the syntax tree. Allows to program with types, generating code branches based on properties of said type. Syntax is easy to understand, and not cryptic like the one of *Rust* for instance.
* 💗  Cross-compilation is simple: Produce an `.o` file, and then use the target-systems
  C compiler to link it into a program. *Note:* The main goal of this is to get the compiler
  to the target platform, to allow compiling on that system. Probably this makes it somewhat
  prohobitive for smaller systems, but that's OK (Rust would be more useful there).
* 💗 It even allows for ruby-like `program` syntax !
* It's a GC'd language, but the authors' are fully aware and provide compile-time constructs to reduce pressure.
* `*varargs` (as tuple) and `**namedargs` (as named tuple), including splat to unpack.
* There is a real `Union` type and *union syntax* (like `String|Int32`).
* Maybe I mentioned it, but the syntax is really optimized to be looking good. It probably is inherited by Ruby, where it matters.
* Fast moving, thanks to small team of dedicated people who know what they are doing.
* external names for functions (like `add(base, by amount) { base + amount }` and called 
  like `add 1, by: 5` to yield `6`)
* Nice time handling: `plan begin: Time.now, end: 2.days.from_now`
* Some support for `private` methods and functions, to controll what goes into modules or
  is callable.
* Support for *interfaces* (called abstract classes, i.e. `abstract class Foo`)
* `abstract` also works for methods in modules, which assures those `include`ing them have to implement them as well.
* flexible module system allowing to `include` and `extend` other modules, types and instances with.
* Support for stack-allocated `struct` types. They are passed by value, and allow nice
  optimizations previously unseen in dynamically-looking languages.
* Blocks are expressions (like in Ruby/Rust)
* Easy integration with C ... it appears at least.
* Standard library provides methods that throw, and those that don't - the latter
  is generally faster, and might even be easier to work with.
* It doesn't implement its own GC, but uses the [BoehmGC](http://www.hboehm.info/gc/) - 
  it's totally conceivable to statically link it to make binaries more portable. It's just
  135kb after all.
  
### Con
* *(maybe)* Binaries need a crystal installation, as they link to some specific libraries that *may* not be present on systems by default (*to be verified*) - compare to `Go`, which has no dependencies at all.
* **compiler** errors are not as helpful as they should be. Compare to *Elm* or even *Rust*
* Looking at the [benchmarks](https://github.com/kostya/crystal-benchmarks-game), memory seems to go very high sometimes ... . Maybe GC is too lazy ?
* Still early in development, v0.18.0 was reviewed, so everything could happen. Not backed by bigger companies yet, it seems.
* No fully-fledged package manager just yet, but `shards` is already there (seems early though).
* No debugging support just yet.
* No destructuring and matching, it seems.

### Notes
* Interesting: Type inference can't determine type of array by what you push into it.
- *No windows support just yet*
- *An exception based language, thanks to ruby heritage*. As [claimed](https://crystal-lang.org/docs/syntax_and_semantics/exception_handling.html)
  it is the preferred way of error handling.
- They refer to `self` even to refer to the type of self (see `Self` in case of Rust,
  which seems more natural).
* It uses single uppercase letters to indicate generic type parameters (i.e. `T`), which
  might actually be nice syntactically.
* I believe to keep them simple, Generics are instantiated when used. However, the error
  messages produces are quite good to understand.

## CoffeeScript

### Pro
* less verbose, densely packed version of javascript
* syntactic sugar for everything
* saves a lot of development time, even though there is
  added complexity

### Con
* initially it's easy to have it generate unintended code by failing to put
  parentheses in the right places
* additional structure might sometimes be preferred to make structure more
  easily graspable.
* it's easy to forget to use `=>` when callbacks are passed from instance
  methods.
* you will not get named functions, which won't help debugging a lot.
* transpiler tends to produce inefficient code for some cases of destructuring. It knows no context.
* when working with classes/this-based objects, it's very easy to forget to use => wherever a callback is used.
  And that even though the added readability is desirable.

###

## C++

### Pro
* **type safety**
* **C++11** adds many languages features that make it into a more useful language with nicer syntax for nowadays common language features.

### Con
* **tool-chain**
    * is not cross-platform at all and usually requires you to bring in additional tools, like **cmake**, to make cross-platform compilation bearable.
    * It is complex with a steep learning curve
    * Builds are generally slow, especially if templates are part of the game. Speeding up build times with precompiled headers is usually not too easy to implement, especially because these shouldn't ever go out of sync in case their headers change.
* **memory-management**
    * There is an awful lot that can go wrong, and writing bullet-proof software is very difficult, requiring you to have mastered the language. Even then, bugs related to memory may sneak in easily.
    * There are automated ways of doing this, but it's an optional feature depending on external libraries, standardized in no way.
* **cross-platform support**
    * Is usually difficult to achieve, requiring a lot of expertise. Compiling on different platforms requires you to deal with entirely different tool-chains, and requires you to acquire builds of the libraries you depend on.
    * Cross-platform compilation can be done, but is not practical
* **unit-tests**
    * There is native support to write and run unit-tests, as well as to run benchmarks.
    * Especially benchmark support is great to spot performance regression over time

## Go

### Pro
* **type safety**
* **rich standard library**
    + It won't have you miss a thing
    + Especially time handling is very well done and feels just right
* **tool-chain**
    * It comes with everything you need to build your go programs, providing facilities to download missing libraries.
    * Builds are extremely fast.
    * Comes with test-driver, profiler, and benchmarker and *documentation generator*
    * Can check for race conditions, which requires your program to run. This in turn, makes good test-cases as necessary as ever.
* **cross-platform support**
    * You can easily run a go source file, making it look more like an interpreted language.
    * Thanks to a nice tool-chain, and a standard library that has cross-platform compatibility in mind, it's easy to support multiple platforms, even though your executable has to be recompiled.
    * Cross-platform compilation is built-in, allowing you to produce binaries for all supported platform on a single system, i.e. build windows and linux binaries from OSX. This works as long as you don't use `cgo`, which by no means is commonly done.
    * Have a look at [gox](https://github.com/mitchellh/gox) for more information
* **performance**
    * It can be as [fast as C](http://blog.golang.org/profiling-go-programs), but is easer to write
    * Memory consumption can [be very close](http://blog.golang.org/profiling-go-programs) to C
    * Executable size is much larger than what C usually is, but it is self-contained, and still pretty decent (around 1MB to 2MB for small programs, which may have around 20KB using c/c++).
        * Build with `go build -ldflags="-s -w"` to reduce the executable size by about 1MB on OSX.
    * It feels rather low-level, which might make it suitable for cross-platform system-programming, or generally, high-performance programming
    * Statically linked self-contained executables are ideal for distribution and startup performance.
    * Has **fast** [regexp](http://swtch.com/~rsc/regexp/regexp1.html)
* **coding style**
    * It encourages a [coding style](http://talks.golang.org/2013/bestpractices.slide) with very short variable names, which makes it harder to understand at first, yet produces rather bulk-free code after all. To me, it's a good thing, it's quite the opposite of what `swift` encourages as well.
    * Weak references not required thanks to mark&sweep garbage collector. Therefore, cycles are an issue of the past.

### Con

* **plugins**
    * There is no dynamic linking, like it's available in c/c++ for example.
    * runtime extension can still work, but would require some form of IPC - goprotobuf and gozmq are available for this purpose. However, there is no pre-made system that could do it.
* **generics**
    * There is no generic support, even though it might be added in the future. A proposed workaround is to use builtin arrays or maps with explicit unboxing.
* **cli libraries**
    * there seems to be no library which can automatically generate high-quality help-text, as arg-parse does it. This is sad, as standalone commandline tools should have been the prime application for go in my thinking.
        + however, even subcommands can be implemented using it, even though you have to work more for it.
    * This [community library](https://github.com/codegangsta/cli) seems to be what you will want to use instead. It supports nested subcommands as well, and generates nice help text.
* **Ideomatic Inconveniences**
    + There is no good way to handle enumerations, apart from C-style constants. Swift does that far better, for instance, as well as C++
    - multi-value returns effectively prevent function chaining, like `f(g())` unless v supports varargs or a slice of interfaces.
        + Nonetheless I do understand it's done on purpose, in order to prevent people from easily ignoring error returns.
    + `x = append(x, elm, ...)` is quite cumbersome. Even though it makes sense and has more applications, it's the most inconvenient form of append I have encountered so far.
    - There is no max/min for `int` types for instance. These are easily implemented, but due to the missing generics, these kind of standard functions will have to be implemented over and over again unless there is some community library to do fill the gap.

## Go 1.6 Reevaluation

### Pro
* All source is considered UTF-8, works even for symbols, except for the first character of a symbol, which must be ascii
* Compiler can choose to put structs on the stack, just leave it to him to make that decision, and you are god
* still: nice c-ish, functional style .... but nothing else expected anyway (only slices and maps can be typed dynamically, using compiler magic)
* **actually** you can use multiple return values ... maybe err destructuring is the only thing that is special
* straight-forward, but possibly hard to read standard function syntax. All the same keywords are used over an over again.

### Cons
* project setup is still cumbersome, and needs the GOPATH to be set, and installations need cryptic `go get ./...` and `go install ./...`
* explicit return, always, makes closures more cumbersome
* still no standard for vendoring, it's not super straightforward to setup a project as you need to put it into the required project structure


### Notes
* one directory equals to one package. Each package can have exactly one main.go


### Interesting

* Callable custom types may be functions, which are declared as type. That way, you can attach methods to functions.


### Resources

* [Effective Go](http://golang.org/doc/effective_go.html)
* [Go FAQ](http://golang.org/doc/faq)


## Swift

### Pro
* Good learning material (iBook, also in html version for free online)
* By default, everything is constant or immutable
    * A great design decision, which removes a class of common programming mistakes and opens up the code for plenty of optimizations.
    * By default, methods may not modify their instance, and anything passed to a function is immutable, unless specified otherwise.

### Con

* Currently only within OSX infrastructure. We will see when exactly there will be open-source implementations. After all, a language is just as useful as the libraries it can use, and currently, this seems to be whatever is available in the `objc` world

### Notes

* So far, the nicest language I have seen, easy to learn, save and fast. Unfortunately, it is a bit too early to use it for anything but OSX/IOS development.


## Dart


### Pro

* Very nice documentation - so far, the best looking

## Python

### Pro

* It's possible to write a lot with a small amount of code ! It's actually quite convenient to write it, common tasks are simple and easily expressed.


## Rust

### Pro

* SAFETY and SPEED at low overhead (if at all)
* Nice documentation thanks to API docs, language reference and book. It's all very well written and a fun read.
* cargo build and deployment system, which is able to handle dependencies correctly (it seems)
    * stackable, hierarchical configuration files to allow overrides - useful for patching existing libraries
    * calls custom build scripts and allow rich integration for text-processing tools, like yacc
    * feature system allows for different builds with a variable feature set, like godi with and without web server gui
* Dynamic loading of plugins
* Very promising and good look API doc generation, including doc-tests ! Allows to embed other languages as well.
* Uses static linking by default ('rlib'), which may be a platform specific default. This will help deployment a lot.
* strings always contain valid unicode, and can contain 0 as they are not null-terminated.
* Great type-inference system to save a lot of typing on the programmers side.
* Great error handling methodology: `let line = std::io::stdin().read_line().ok().expect("Need stdin to work")`
* extremely powerful and versatile enumerations !
* Generics ! Everywhere ! Allow to parameterize types and select where to apply implementations using traits.
* Traits are used as Interface definitions and markers to apply generic implementations to. That's the way to avoid typical issues with hierarchical taxonomies. It's mixins in other languages !
* Powerful macro system for meta-programming/code generation (to what extend ?)
* Incredibly powerful and useful pattern matching with `match` ... it's even more powerful than I thought as it allows you to achieve auto-dereferencing.
* Supports valgrind
* supports channels, iteration over (open) channels, and select on timers and channels.
* The more unsafe a data source is, there more difficult it is to access it: more checking is needed.
* Most expression oriented language, and conditionals like `if` and `match` have a return value to assign to LHS or for use as function return value.

### Con

* **It's ALPHA and constantly changing**. Lot's of existing code out there is outdated, at least if it's not libraries that are meant to stay, like `piston`. The latter also changes a lot, which doesn't help ;).
* Even though cross-platform compilation is possible, it's not yet implemented conveniently (like `gox` in `go-lang`.
* somewhat steep learning curve
* The borrow checker seems greedy, and sometimes flags issues which are actually none.
* Traits need to be imported to be usable, even in situations where there is no ambiguity. This leads to import terror.
* (traits are needed to make any type usable comfortably in generics. Doing that makes them less comfortable)
* PhantomData: "the type parameter `D` is not constrained by the impl trait, self type, or predicates", makes using other generic types more cumbersome, and just a chore.

### Confusion

 * r: &item
   * Access `r.member`, but `let x: *r`.
 * `fn foo(b: &bar)` is called like `foo(&b)`, explicit pointer-of operator, even though a reference is created. The C++ person inside of me has to be silenced ;).

# Comparison

Please note that the following table will look correctly only with github flavored markdown.
[Click here to view](https://github.com/Byron/depot/blob/master/src/md/lang.md).

## Language Features

Lang         | Type-Safety | Generics | Exceptions | Garbage Collection | MT-Support | Closures | Performance | Plugins | Reflection | C Interop | Const | Destructor |
------------ | ----------- | -------- | ---------- | ------------------ | ---------- | -------- | ----------- | ------- | ---------- | --------- | ----- | ---------- |
python       | ✘           | (✓)      | ✓          | ✓                  | ★☆☆        |  ✓       | ★☆☆         | ✓       |  ✓         |  ✓        | ✘     | ✓          |
cpp          | ✓           |  ✓       | ✓          | ✘ (✓)              | ★★☆        |  ✓       | ★★★         | ✓       | (✓)        |  ✓        | ✓     | ✓          |
go           | ✓           |  ✘       | ✘          | ✓                  | ★★★        |  ✓       | ★★★         | ✘       |  ✓         | (✓)       | ❍     | ✘          |
dart         | ❍ (✓)       |  ❍       | ❍          | ✓                  | ❍          |  ✓       | ★★☆         | ✓       |  ❍         |  ❍        | ❍     | ❍          |
swift        | ✓           |  ✓       | ✘          | ✓                  | ★★☆        |  ✓       | ★★★         | ❍       | (✓)        |  ✓        | ✓     | ✓          |
rust         | ✓           |  ✓       | ✘          | ✘ (✓)              | ★★★        |  ✓       | ★★★         | ✓       | statictyps | ✓         | ✓     | ✓          |


## Tool Chain Features

Lang         | CPU Profiler | Memory Profiler | CP Executables | CP Compilation | Debugger | IDE | Unit Testing | YAML|ZMQ|QT |
------------ | ------------ | --------------- | -------------- | -------------- | -------- | --- | ------------ | ----------- |
python       | ✓            | ✘               | ✓              | -              | ✓        |  ✓  |  ✓           | ✓ | ✓ | ✓   |
cpp          | ✓            | ✓               | ✘              | ✘              | ✓        |  ✓  | (✓)          | ✓ | ✓ | ✓   |
go           | ✓            | ✓               | ✘              | ✓              | ✘        | (✓) |  ✓           | ✓ | ✓ | ✘   |
dart         | ❍            | ❍               | ✓              | -              | ❍        |  ✓  |  ❍           | ✓ | ✘ | ✘   |
swift        | ✓            | ✓               | ✘              | ✘              | ✓        |  ✓  |  ❍           | ✘ | ✘ | ✘   |
rust         | ?            | ✓               | ✘              | (✓)            | ?        |  ✘  |  ✓           | ✓ | ✓ | ✘   |

* **Legend**
    * ✘  : not supported
    * ✓  : supported
    * -  : doesn't apply
    * (✓): feature is not perfectly applicable or not natively supported, yet effectively supported, possibly through external libraries
    * ❍ (✘|✓) : not sure (with tendency to (not) supported)
    * ★☆ : rating - see feature list for more information

t* **Features**
    * *Type Safety*
        * Expected types in function signatures and for variables are explicitly defined and/or matter. This allows for compile-time checking of types to assure they are used correctly.
        * Opens up your code for static code analysis, and will help your IDE to auto-complete and hint, generally boosting comfort and convenience, easing the learning curve.
        * If this is not the case, everything is determined at runtime, adding an entire class of possible errors, as common in python, javascript and ruby for example.
        * On the other hand, code written in languages which are not type-safe is less verbose and thus faster to write in theory. In practice, type information should be provided if others should be able to use your API.
    * *Generics*
        * Allows to write types and algorithms which can work with any type that provides certain capabilities. The actual type is generated based on which type the generic should specialize in. It's as efficient as if it would have been implemented by hand, but adds cost to the compile time of the program.
        * A very effective mean to prevent code duplication, or to be forced to add code-generators into the tool chain.
    * *Exceptions*
        * Provide an alternate return path for functions, effectively allowing clients of your API to know that they will get a valid return value, or none at all.
        * Effectively, those who don't care about exceptions don't handle them, and can write code which doesn't check for the validity of return values.
        * Those who are handling exceptions, will have to write more verbose code compared to a simple `if` clause.
        * Exceptions are sometimes disputed, and even though I use them extensively, they do add complexity to your API as people have to know which exceptions can be thrown, if any, and what return values can be - after all, some functions do return `nil`, `None` or `0` to indicate an error condition.
    * *Garbage Collection*
        * The term is used to indicate that memory management is automated. This means, you will never have to deal with memory as a resources, with acquiring or releasing it, which removes an entire class of programming errors.
        * It also makes leakage through cyclic links easy, which requires the programmer to at least be aware of how it works.
    * *MT-Support*
        * Multi-threading support
        * ★☆☆
            * Native threads - threads that actually run concurrently, as orchestrated by the operating system kernel
        * ★★☆
            * Concurrent Code - code can run concurrently, as it's not blocked by (automatic) mutexes. This is the case in some interpreted languages, and hampers the usefulness and performance gains of multiple threads.
        * ★★★
            * Built-in concurrency primitives - the language is built for running concurrently, and makes it easy to use through built-in language features.
    * *Closures*
        * See [wiki](http://en.wikipedia.org/wiki/Closure_(computer_programming))
        * All programming languages support some form of closure, even though C++ doesn't natively support binding of surrounding data. This functionality can be added through boost, but comes with complex syntax and at the expense of templates. C11 does have [native closure support](http://www.cprogramming.com/c++11/c++11-lambda-closures.html).
    * *Performance*
        * Describes the overall CPU performance and memory efficiency of the language
        * ★☆☆
            * Slow (interpreted) execution without JIT, high memory overhead as everything is an object, without any optimizations. Startup performance of interpreted programs is high as it involves a lot of IO, e.g. reading of various files. Examples are `python` and `ruby`
        * ★★☆
            * Fast possibly interpreted execution, usually achieving close-to-C performance using a JIT in certain regions of the code. Memory overhead is still high, and startup times are high due to interpreter overhead. Examples are `javascript`, `dart` and `java`
        * ★★★
            * Optimized, platform dependent machine code which runs as fast as C, memory overhead is low and close to what a good C program can accomplish. Startup time of the executable is low as it can be executed directly.
    * *Plugins*
        * A programs ability to load additional code at runtime without modifying the original executable. This code is called a plugin, and usually provided as binary (c++, python byte code) or text file (interpreted languages).
    * *Reflection*
        * Allows a program to examine the type of otherwise unknown objects.
        * (✓) indicates that this runtime type information only includes type information, but not the methods and attributes of that type.
    * *C Interop*
        * Allows the language to be extended using C or C++ programs, and to make calls to C/C++ libraries. This is somewhat important in case performance is a premise.
        * (y) means that there are restrictions, and that only C/C++ calls can be done (so there is no C-extension possible).
    * *Const*
        * A language that allows to specify instances as constant, which effectively makes them read-only, in one way or another.
        * This feature helps to prevent plenty of programming errors, and makes programs safer.
    * ** *Destructor**
        * Allows for an instance to execute some clean-up code when no one is referencing it anymore. Useful for assured cleanup.
    * *CP Executables*
        * Cross-platform executables per se don't exist, which is why something similar to cross-platform executables only works for interpreted, byte-compiled languages. Those are python, ruby, javascript, and java for example.
        * Everything else is compiled to machine-code, and as such depends heavily on the platform it was compiled on.
    * *CP Compilation*
        * Cross-platform compilation allows to generate executables for multiple platforms, on a single source platform.
        * Interpreted languages, like python, java, ruby, support this natively, even though they might depend on a certain interpreter version to run. One byte-code file will natively run on multiple platforms that way.
    * *CPU Profiler*
        * A program to trace how much time is spend in a function, with support for reporting and possibly visualization
    * *Memory Profiler*
        * A tool to track memory allocation and deallocations, per function, and to detect leakage
    * *Debugger*
        * A standard tool to break execution of a program, allowing to introspect all of its state and call stack.
    * *IDE*
        * An integrated development environment, may come in the form of a custom program, extensions to editors like sublime text, vim or emacs.
    * *Unittesting*
        * Facilities to declare test cases and run them, displaying the result in a fashion helping to debug the problem.

# Conclusion
