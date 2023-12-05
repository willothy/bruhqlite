use std::{io::Write, str::FromStr};

use chumsky::{
    primitive::{any, choice, just, one_of},
    text::{self, digits, ident, inline_whitespace},
    ParseResult, Parser,
};

enum MetaCommand {
    Exit,
}

enum Statement {
    Insert {
        id: u32,
        username: String,
        email: String,
    },
    Select,
}

enum Command {
    Meta(MetaCommand),
    Statement(Statement),
}

fn metacommand<'a>() -> impl Parser<'a, &'a str, MetaCommand> {
    just(".").ignore_then(choice((
        text::keyword("exit").map(|_| MetaCommand::Exit),
        // other commands here
    )))
}

fn email<'a>() -> impl Parser<'a, &'a str, String> {
    text::ident()
        .then_ignore(just("@"))
        .then(text::ident())
        .then_ignore(just("."))
        .then(text::ident())
        .map(|((name, domain), ext)| format!("{}@{}.{}", name, domain, ext))
}

fn insert<'a>() -> impl Parser<'a, &'a str, Statement> {
    text::keyword("insert")
        .then_ignore(inline_whitespace())
        .then(digits(10))
        .map(|(id, _): (&str, _)| id.parse::<u32>().unwrap())
        .then_ignore(inline_whitespace())
        .then(ident().map(ToOwned::to_owned))
        .then(email())
        .map(|((id, username), email)| Statement::Insert {
            id,
            username,
            email,
        })
}

fn select<'a>() -> impl Parser<'a, &'a str, Statement> {
    text::keyword("select")
        .ignored()
        .then_ignore(any().repeated())
        .map(|_| Statement::Select)
}

fn statement<'a>() -> impl Parser<'a, &'a str, Statement> {
    choice((
        insert(),
        select(),
        // other statements here
    ))
}

fn parser<'a>() -> impl Parser<'a, &'a str, Command> {
    choice((
        metacommand().map(Command::Meta),
        statement().map(Command::Statement),
    ))
}

fn main() {
    let mut buffer = String::new();

    'mainloop: loop {
        print!("db > ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut buffer).unwrap();

        let output = parser().parse(buffer.trim());
        if output.has_errors() {
            for error in output.errors() {
                println!("{}", error);
            }
            continue 'mainloop;
        }

        if let Some(command) = output.output() {
            match command {
                Command::Meta(meta) => match meta {
                    MetaCommand::Exit => break 'mainloop,
                },
                Command::Statement(stmt) => match stmt {
                    Statement::Insert {
                        id,
                        username,
                        email,
                    } => todo!(),
                    Statement::Select => todo!(),
                },
            }
        }

        // buffer.clear();
    }
}
