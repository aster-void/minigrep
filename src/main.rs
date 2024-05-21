use std::io;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let Ok(config) = minigrep::parse_config(std::env::args()) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "not enough arguments",
        ));
    };
    let content = if let Some(filename) = config.filename {
        let mut f = std::fs::File::open(filename)?;
        let mut c = String::new();
        f.read_to_string(&mut c)?;
        Some(c)
    } else {
        None
    };
    if let Some(content) = content {
        // filename found: doing file search
        let result = minigrep::search_string(&config.query, &content);
        println!("{}", result.join("\n"));
    } else {
        // filename not found: doing stdin search
        let stdin = std::io::stdin();
        let mut buf = String::new();
        while let Ok(_) = stdin.read_line(&mut buf) {
            if minigrep::search_in_line(&config.query, &buf) {
                println!("{}", buf);
            }
        }
    }
    Ok(())
}
