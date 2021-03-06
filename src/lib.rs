#![deny(clippy::disallowed_types)]

mod compile;
mod errors;
mod runtime;
mod syntax;
mod util;

use std::io::Write;

pub use bumpalo::Bump;

use crate::{
    errors::CompilerError,
    syntax::{ast::Program, lex, parse},
};
pub use crate::{
    runtime::gc::RtAlloc,
    syntax::{lex::*, parse::*},
};

#[cfg(not(feature = "fxhash"))]
#[allow(clippy::disallowed_types)]
type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(feature = "fxhash")]
type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

#[cfg(not(feature = "fxhash"))]
#[allow(clippy::disallowed_types)]
type HashSet<T> = std::collections::HashSet<T>;

#[cfg(feature = "fxhash")]
type HashSet<T> = rustc_hash::FxHashSet<T>;

pub struct Config<'io> {
    pub debug: bool,
    pub step: bool,
    pub parse_only: bool,
    pub stdout: &'io mut dyn Write,
}

pub fn run_program(program: &str, cfg: &mut Config) {
    if cfg.debug {
        eprintln!("Config: debug: {}, step: {}", cfg.debug, cfg.step);
    }

    let ast_alloc = Bump::new();

    // SAFETY: I will try to 🥺
    let mut runtime = unsafe { RtAlloc::new() };

    let lexer = Lexer::new(program, &mut runtime);
    let ast = parse::parse(lexer, &ast_alloc);

    match ast {
        Ok(ast) => {
            if cfg.parse_only {
                return;
            }
            process_ast(program, &ast, runtime, cfg)
        }
        Err(err) => errors::display_error(program, err),
    }
}

fn process_ast(program: &str, ast: &Program, mut runtime: RtAlloc, cfg: &mut Config<'_>) {
    if cfg.debug {
        util::dbg("AST:\n", ast);
    }

    let bytecode_alloc = Bump::new();

    let bytecode = compile::compile(ast, &bytecode_alloc, &mut runtime);

    match bytecode {
        Ok(code) => {
            if cfg.debug {
                util::dbg("Bytecode:\n", code);
            }

            let result = runtime::vm::execute(code, runtime, cfg);
            if let Err(msg) = result {
                eprintln!("error: {msg}");
            }
        }
        Err(err) => errors::display_error(program, err),
    }
}

// have the code here and not in the fuzzer, it's easier to find when it breaks like this

#[doc(hidden)]
pub fn _fuzz_compile(program: &str) {
    // SAFETY: Just this scope
    let mut runtime = unsafe { RtAlloc::new() };
    let ast_alloc = Bump::new();

    let lexer = lex::Lexer::new(program, &mut runtime);
    let ast = parse::parse(lexer, &ast_alloc);

    if let Ok(ast) = ast {
        let bytecode_alloc = Bump::new();
        let _bytecode = compile::compile(&ast, &bytecode_alloc, &mut runtime);
    }
}

#[doc(hidden)]
pub fn _fuzz_parse(program: &str) {
    // SAFETY: Just this scope
    let mut runtime = unsafe { RtAlloc::new() };
    let ast_alloc = Bump::new();

    let lexer = lex::Lexer::new(program, &mut runtime);
    let _ast = parse::parse(lexer, &ast_alloc);
}

#[doc(hidden)]
pub fn _fuzz_lex(program: &str) {
    // SAFETY: Just this scope
    let mut runtime = unsafe { RtAlloc::new() };
    let lexer = lex::Lexer::new(program, &mut runtime);
    for _token in lexer {}
}

#[doc(hidden)]
pub fn _parse<'ast>(
    program: &str,
    alloc: &'ast Bump,
    rt_alloc: &mut RtAlloc,
) -> Result<Program<'ast>, CompilerError> {
    let lex = Lexer::new(program, rt_alloc);
    parse(lex, alloc)
}
