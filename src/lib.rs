#![allow(clippy::needless_return)]

//! Rocks is a programming language written in Rust. It is a dynamically typed language with
//! lexical scoping and first-class functions. Rocks is a tree-walk interpreter with a hand-written
//! recursive descent parser. Rocks is a hobby project and is not intended for production use.
//!
//! Rocks is a dynamically typed language. This means that the type of a variable is determined at
//! runtime. This is in contrast to statically typed languages, where the type of a variable is
//! determined at compile time. Dynamically typed languages are often easier to use, but are
//! generally slower than statically typed languages.
//!
//! Rocks is a tree-walk interpreter. This means that the interpreter walks the abstract syntax tree
//! (AST) and evaluates each node. This is in contrast to a compiler, which would convert the AST
//! into bytecode or machine code. Tree-walk interpreters are generally easier to implement than
//! compilers, but are generally slower than compilers.
//!
//! Rocks is a hobby project and is not intended for production use. The goal of this project is to
//! learn more about programming languages and interpreters. This project is inspired by the
//! [Crafting Interpreters](https://craftinginterpreters.com/) book by Bob Nystrom.
//!
//! ## Scanning
//! The first step in the interpreter is scanning. Scanning is the process of converting a string of
//! characters into a list of tokens. A token is a single unit of a programming language. For
//! example, the string `1 + 2` would be converted into the following tokens:
//! ```text
//! [Number(1), Plus, Number(2)]
//! ```
//! The scanner is implemented in the [`scanner`](scanner) module as an iterator over the characters
//! in the source code. It is a simple state machine that returns the next token in the source code
//! when called.
//!
//! The scanner reports syntax errors in the source code as a [`ScanError`](error::ScanError).
//! These errors are trivial problems like an unterminated string literal or an unexpected character.
//! Scan errors are reported as soon as they are encountered. This means that the scanner will
//! continue scanning the source code even if it has already encountered a syntax error. This is
//! useful because it allows the user to fix multiple syntax errors at once.
//!
//! ## Parsing
//! The second step in the interpreter is parsing. Parsing is the process of converting a list of
//! tokens into an abstract syntax tree (AST). The parser is implemented in the [`parser`](parser)
//! module as a recursive descent parser. The parser transforms the list of tokens into expressions
//! and statements. [`Expressions`](expr::Expr) are pieces of code that produce a value, specifically an
//! [`Object`](object::Object). Objects are an umbrella term for all types of values in Rocks
//! including literals, functions, classes and instances. [`Statements`](stmt::Stmt) are pieces of code
//! that do not produce a value but instead perform some action. These actions modify the state of the
//! program and thus, are called side-effects. For example, a variable decleration or an if clause
//! would be classified as statements.
//!
//! For example, the string `print 1 + 2;` would be converted into the following AST:
//! ```text
//! PrintStatement {
//!     BinaryExpression {
//!         left: Number(1),
//!         operator: Plus,
//!         right: Number(2),
//!     }
//! }
//! ```
//! The parser reports syntax errors in the source code as a [`ParseError`](error::ParseError).
//! Unlike the scanner, the parser catches errors that span multiple tokens. For example, the
//! following expression is invalid because it is missing the right-hand operand:
//! ```text
//! 1 !=
//! ```
//! However, much like the scanner, the parser will continue parsing the source code even if it
//! has already encountered a syntax error using a technique called synchronization. This is useful
//! because it allows the user to fix multiple syntax errors at once.
//!
//! ## Resolving
//! The third step in the interpreter is resolving. Resolving is the process of statically analyzing
//! the AST to determine the scope of each variable. While this requires a pre-pass of the AST, it
//! is necessary to construct robust lexiacl scoping. The resolver is implemented in the
//! [`resolver`](resolver) module as a tree-walk interpreter. The resolver is run after the parser
//! because it requires the AST to be fully constructed. The resolver reports errors as a
//! [`ResolveError`](error::ResolveError). These errors are syntactically valid but semantically invalid.
//! and therefore, cannot be caught by the scanner or the parser. For example, the following expression
//! is valid a valid Rocks syntax but it is semantically invalid because the variable `a` is defined
//! twice in the same scope:
//! ```text
//! {
//!    var a = 1;
//!    var a = 2;
//! }
//! ```
//!
//! ## Interpreting
//! The final step in the interpreter is _interpreting_. Interpreting is the process of evaluating the
//! AST. The interpreter is implemented in the [`interpreter`](interpreter) module as a tree-walk
//! interpreter. Thanks to all the previous steps, the interpreter is able to evaluate the AST and produce
//! a result. The interpreter reports errors as a [`RuntimeError`](error::RuntimeError). While the
//! scanner, the parser and the resolver try to catch as many errors as possible before running the
//! code, most errors can only be caught at runtime. For example, the following expression is valid
//! Rocks syntax but it is semantically invalid because it tries to add a string and a number:
//! ```text
//! var a = "123";
//! var b = a + 123;
//! ```
//! The interpreter is also responsible for managing the environment. The environment is a mapping of
//! variable names to their values. The environment is implemented in the [`environment`](environment)
//! module as a stack of hash maps. Each hash map represents a scope in the program. This allows the
//! interpreter to implement lexical scoping. The interpreter also manages the call stack.

use std::{fs, process};
use std::io::{self, Write};

pub mod error;
pub mod token;
pub mod scanner;
pub mod expr;
pub mod stmt;
pub mod environment;
pub mod parser;
pub mod ast;
pub mod interpreter;
pub mod literal;
pub mod object;
pub mod function;
pub mod resolver;
pub mod class;

use parser::Parser;
use scanner::Scanner;
use resolver::Resolver;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

#[allow(non_camel_case_types)]
pub struct rocks {
    interpreter: interpreter::Interpreter,
}

impl rocks {
    pub fn new() -> Self {
        rocks {
            interpreter: interpreter::Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, path: String) {
        let contents = fs::read_to_string(path)
            .expect("Should have been able to read the file");

        self.run(contents);

        unsafe {
            if HAD_ERROR {
                process::exit(65);
            }
            if HAD_RUNTIME_ERROR {
                process::exit(70);
            }
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();
            io::stdout().write_all(b"> ").unwrap();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("acceptable expression");

            self.run(input);

            unsafe {
                HAD_ERROR = false;
                HAD_RUNTIME_ERROR = false;
            }
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        if error::did_error() {
            return;
        }

        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        if error::did_error() {
            return;
        }

        let mut resolver = Resolver::new(&mut self.interpreter);
        resolver.resolve(&statements);

        if error::did_error() {
            return;
        }

        self.interpreter.interpret(&statements);
    }
}

impl Default for rocks {
    fn default() -> Self {
        Self::new()
    }
}
