#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn one_result() {
        let key = "duct";
        let content = "\
        Rust.
        Fast, Safe and Productive.
        Pick three.";
        assert_eq!(
            search_string(key, content),
            vec!["Fast, Safe and Productive"]
        );
    }
}

pub fn search_string(key: &str, pool: &str) -> Vec<String> {
    return search(key, pool.lines());
}
pub fn search_in_line(key: &str, line: &str) -> bool {
    return line.contains(key);
}

pub fn search<'a, S>(key: &str, pool: S) -> Vec<String>
where
    S: 'a + Iterator<Item = &'a str>,
{
    return pool
        .filter(|s| s.contains(key))
        .map(|s| s.to_owned())
        .collect();
}

pub struct Config {
    pub query: String,
    pub filename: Option<String>,
}

pub enum ParseConfigError {
    NotEnoughArguments,
    InvalidArguments,
}

pub fn parse_config(mut args: std::env::Args) -> Result<Config, ParseConfigError> {
    let query = args.next().ok_or(ParseConfigError::NotEnoughArguments)?;
    let Some(filename) = args.next() else {
        return Ok(Config {
            query,
            filename: None,
        });
    };
    return Ok(Config {
        query,
        filename: Some(filename),
    });
}
