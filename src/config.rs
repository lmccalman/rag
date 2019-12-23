use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Config {
    pub outfile: String,
    pub infile: String,
}

impl Config {
    pub fn new() -> Result<Config> {

    let argvec: Vec<String> = std::env::args().collect();
    let l = argvec.len();
        if l == 3 {
            let outfile = argvec[1].clone();
            let infile = argvec[2].clone();
            Ok(Config { outfile, infile })
        } else
        {
            return Err(anyhow!("Wrong number of arguments: found {} needed 2", l - 1));
        }
    }
}
