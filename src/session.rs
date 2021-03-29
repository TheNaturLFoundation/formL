use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Session {
    editor: Editor<()>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            editor: Editor::<()>::new(),
        }
    }

    pub fn start_session(&mut self) {
        loop {
            let readline = self.editor.readline(">> ");
            match readline {
                Ok(line) => {
                    self.editor.add_history_entry(line.as_str());
                    println!("{}", line);
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
