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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n = cin.next::<u64>();
    if n % 4 == 2 || n % 4 == 1 {
        writeln!(cout, "NO").ok();
    } else {
        writeln!(cout, "YES").ok();
        if n % 4 == 0 {
            writeln!(cout, "{}", n / 2).ok();
            for i in 0..n / 4 {
                write!(cout, "{} {} ", i + 1, n - i).ok();
            }
            writeln!(cout, "").ok();
            writeln!(cout, "{}", n / 2).ok();
            for i in n / 4..n / 2 {
                write!(cout, "{} {} ", i + 1, n - i).ok();
            }
            writeln!(cout, "").ok();
        } else {
            writeln!(cout, "{}", (n + 1) / 2).ok();
            for i in 0..(n + 1) / 4 {
                write!(cout, "{} {} ", i + 1, n - 1 - i).ok();
            }
            writeln!(cout, "").ok();
            writeln!(cout, "{}", n / 2).ok();
            for i in (n + 1) / 4..n / 2 {
                write!(cout, "{} {} ", i + 1, n - 1 - i).ok();
            }
            writeln!(cout, "{}", n).ok();
        }
    }
}
