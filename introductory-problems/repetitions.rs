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
        line
    }
}

pub fn group_by<I, T>(xs: I) -> Vec<(T, usize)>
where
    I: Iterator<Item = T>,
    T: Eq + PartialEq,
{
    let mut groups = Vec::<(T, usize)>::new();
    for item in xs {
        let last = groups.last_mut();
        if let Some((_, cnt)) = last.filter(|(k, _)| k == &item) {
            *cnt += 1;
        } else {
            groups.push((item, 1));
        }
    }
    groups
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let s = cin.next_line();
    let s = s.as_bytes();
    let ret = group_by(s.iter()).iter().map(|(_, c)| *c).max().unwrap();

    writeln!(cout, "{}", ret).ok();
}
