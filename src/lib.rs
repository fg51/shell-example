use std::io::Write;

pub struct LshLoop;

impl LshLoop {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        loop {
            print!("> ");
            let mut stdout = std::io::stdout();
            stdout.flush()?;

            let line = read_line()?;
            let tokens = split_line(line);
            let _status = execute(tokens);
            //match status {

            //}
        }
    }
}

fn read_line() -> Result<String, std::io::Error> {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    return Ok(line);
}

fn split_line(_line: String) -> Vec<String> {
    // return tokens;
    todo!();
}

fn execute(_tokens: Vec<String>) -> Vec<String> {
    // return status;
    todo!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
