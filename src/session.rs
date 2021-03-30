use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::processing::parser::Parser;

pub struct Session {
    editor: Editor<()>,
    parser: Parser,
}

impl Session {
    pub fn new() -> Session {
        Session {
            editor: Editor::<()>::new(),
            parser: Parser::new(),
        }
    }

    pub fn start_session(&mut self) {
        loop {
            let readline = self.editor.readline(">> ");
            match readline {
                Ok(line) => {
                    self.editor.add_history_entry(line.as_str());
                    let expr = self.parser.parse(&line);
                    dbg!("{}", *expr);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
