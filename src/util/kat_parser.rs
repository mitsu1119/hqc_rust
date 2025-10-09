use std::{
    fs::File,
    io::{self, BufRead, BufReader},
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

    fn hex_to_bytes(s: &'a str) -> Vec<u8> {
        assert_eq!(s.len() & 1, 0);
        let mut out = Vec::with_capacity(s.len() >> 1);
        for i in (0..s.len()).step_by(2) {
            let byte = u8::from_str_radix(&s[i..i + 2], 16).expect("");
            out.push(byte);
        }
        out
    }

    pub fn bytes_after(&mut self, s: &'a str) -> io::Result<Option<Vec<u8>>> {
        let bytes = self.line_after(s)?;
        if let Some(b) = bytes {
            Ok(Some(Self::hex_to_bytes(&b)))
        } else {
            Ok(None)
        }
    }
}
