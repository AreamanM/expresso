use rustyline::error::ReadlineError;
use rustyline::Editor;
use colored::*;

use cocoa::{lexer::lex, parser::parse};

fn main() {
    repl();
}

fn repl() {
    println!(r#"expresso REPL v0.1.0

Enter expressions to see their answer or press Ctrl-d to quit.
"#);

    let mut rl = match Editor::<()>::new() {
        Ok(editor) => editor,
        Err(e) => {
            println!("{}", e.to_string().red());
            return;
        }
    };

    loop {
        let line = rl.readline("> ");

        match line {
            Ok(line) => {
                let tokens = match lex(&mut line.chars().peekable()) {
                    Ok(ts) => ts,
                    Err(e) => {
                        println!("{}", e.to_string().red());
                        continue;
                    }
                };

                match parse(&mut tokens.into_iter().peekable(), 0) {
                    Ok(n) => println!("{}", n),
                    Err(e) => println!("{}", e.to_string().red()),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("{}", err.to_string().red());
                break;
            }
        }
    }
}
