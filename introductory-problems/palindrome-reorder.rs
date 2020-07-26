use std::io::{self, Write};

struct Scanner<R> {
    reader: R,
}

impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self { reader }
    }

    fn next_line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).expect("Failed to read line");
        if line.ends_with('\n') {
            line.pop();
        }
        line
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let s = cin.next_line();
    let mut cnt = vec![0; 26];
    for c in s.as_bytes() {
        cnt[(c - 65) as usize] += 1;
    }
    let mut ret = Vec::<char>::new();
    let mut last = None;
    for c in 0..26 {
        for _ in 0..cnt[c] / 2 {
            ret.push(char::from(c as u8 + 65));
        }
        if cnt[c] % 2 == 1 {
            if last.is_some() {
                break;
            } else {
                last = Some(char::from(c as u8 + 65));
            }
        }
    }
    let mut len = ret.len() * 2;
    if last.is_some() {
        len += 1;
    }
    if len != s.len() {
        writeln!(cout, "NO SOLUTION").ok();
    } else {
        for c in ret.iter() {
            write!(cout, "{}", c).ok();
        }
        if let Some(c) = last {
            write!(cout, "{}", c).ok();
        }
        for c in ret.iter().rev() {
            write!(cout, "{}", c).ok();
        }
        writeln!(cout, "").ok();
    }
}
