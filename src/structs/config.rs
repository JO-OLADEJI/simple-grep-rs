#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(x) => x,
            None => return Err("`query` not found!"),
        };
        let file_path = match args.next() {
            Some(y) => y,
            None => return Err("`file_path` not found!"),
        };
        let ignore_case = match args.next() {
            Some(z) => {
                if z == "-i" {
                    true
                } else {
                    false
                }
            }
            None => false,
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
