# Basic Calculator

This was my first attempt at doing anything of use with the Rust
programming language. As a result, I chose a very simple problem
statement that'll help me in that learning process, instead of having
to fiddle around with the business logic particulars of the domain.

# Problem Statement

I first came across the basic calculator problem statement in the
[Peer to Peer](http://peertopeer.io)
[session](http://peertopeer.io/videos/4-john-wiegley/) between Ollie
Charles and John Wiegley. For such a simple problem statement, I was
impressed by how many Haskell features were demonstrated in that
video. That is why I decided to use it as my first problem statement
in learning Rust.

The [problem statement](problem_statement.pdf) itself is available as
part of this codebase.

## Changes In The Problem Statement Language

- All statements within a block (anything within opening ({) and
  closing (}) braces) must end with a semi-colon. Without this,
  parsing became unnecessarily complex. Adding a delimiter simplified
  parsing a lot. As a result of this, even *if* statements within a
  function need to end with a semi-colon.

# Solution

I decided to use [nom](https://github.com/Geal/nom) for the parsing
since it seemed to be the most popular parsing library in Rust. Plus,
on using it for the first time, I quickly learned that it's macros and
DSL for building parser combinators was very easy and expressive to
use. In fact, if you look at the parser portion of the codebase, I
hope it is pretty much self-explanatory.

Taking cue from the P2P video, I decided to build an AST after parsing
instead of doing evaluation inline, as in one of the Nom examples.
Obviously this is also needed if you're going to handle *let
expressions* and *function definitions*.

# Rust Language Features Displayed in the Codebase

I wanted this project to be as beginner friendly as possible.
Basically, if you look at this codebase as your first Rust project
right after reading the Rust Book, you should be able to understand
pretty much everything going on here.

To that end, this codebase makes use of the following Rust idioms, all
of which are covered in the Rust Book:

- **ENums** - The entire basic calculator language AST is just a Rust
  ENum. Note also the usage of the *Box* trait in the recursive ENum.
- **Structs** - Rust structs are used in some places to hold the
  result of parsed expressions. This proved to be very handy in
  constructing an AST for IF statements.
- **Result and the Question Mark** (*?*) macro - Rust's Result type
  proved to be very useful, especially when used hand-in-hand with the
  *?* macro as well. You can see idiom used extensively in the
  evaluator.
- **Display trait** - This trait is implemented for our Error type to
  get nice error messages when evaluation of an expression fails.
- **Tests** - Rust has a wonderful and easy-to-use unit testing
  mechanism cooked into the language itself. This means you don't have
  to add any external crates for the same. In all places, unit tests
  are placed next to the code under test in a separate *tests* module.
  This is in accordance with Rust best practices.

In addition, it also uses the following **Nom** idioms:

- **do_parse!** - This is the bread and butter of using Nom. It is
  used extensively to construct small parsers and build on top of
  them. It has a very nice DSL that makes it clearly apparent what the
  parser is doing.
- **Producer and Consumer** - Nom's streaming capabilities proved to
  be really handy in implementing parsing from a file. Most of the
  implementation is directly copied over from the [Web Archive parser
  example](https://github.com/sbeckeriv/warc_nom_parser/blob/dc950b7cfa76eda0ea1bf188f7c344103a6274e4/src/lib.rs#L29)
  linked to in the Nom Readme.

# TODOs

- [ ] Avoid cloning. As a result of passing around ownership of the
      expression, there's a lot of cloning and memory inefficiency
      going on. I'm sure this can be avoided. (Note that cloning of
      environment during function calls is necessary to avoid mutating
      the global environment.)
- [ ] Add a multi-line REPL. Right now, the REPL is capable of reading
      an entire BC statement from a single line only. This can also be
      improved.
- [ ] Evaluating an IF statement can also be improved. The current
      solution is not very easy to read and understand.
- [ ] Lazy Loading of variables - The ELet statements can be memoized
      to compute the result and store only on first use, instead of on
      definition.
- [ ] Avoid Returning Values for Let and Define - Since the parser
      expects f32 values to be returned, we're unnecessarily returning
      values for both these statements. This can also be avoided.

# LICENSE

This project is licensed under Mozilla Public License 2.0. For more
details, see the full LICENSE file.
