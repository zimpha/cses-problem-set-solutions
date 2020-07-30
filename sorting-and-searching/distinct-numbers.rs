use std::io::{self, Write};
use std::str;

struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_ascii_whitespace(),
        }
    }
    fn next<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }

    fn next_vec_n<T: str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
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

    let n: usize = cin.next();
    let mut x: Vec<u32> = cin.next_vec_n(n);
    x.sort();

    writeln!(cout, "{}", group_by(x.iter()).len()).ok();
}
