use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

#[derive(Debug)]
pub struct KATParser {
    br: BufReader<File>,
}

impl<'a> KATParser {
    pub fn new(fname: &'a str) -> io::Result<Self> {
        let file = File::open(fname)?;
        let br = BufReader::new(file);
        Ok(Self { br })
    }

    pub fn line_after(&mut self, s: &'a str) -> io::Result<Option<String>> {
        let mut res = String::new();

        loop {
            res.clear();
            if self.br.read_line(&mut res)? == 0 {
                return Ok(None);
            }
            if let Some(pos) = res.find(s) {
                let mut ss = res[pos + s.len()..].to_string();
                if ss.ends_with("\n") {
                    ss.pop();
                }
                if ss.ends_with("\r") {
                    ss.pop();
                }
                return Ok(Some(ss));
            }
        }
    }
}
