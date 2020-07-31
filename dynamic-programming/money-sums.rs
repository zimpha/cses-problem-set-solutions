use std::{
    io::{self, Write},
    str,
};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n: usize = cin.next();
    let x: Vec<usize> = cin.next_vec_n(n);
    let sx: usize = x.iter().sum();
    let mut dp = vec![false; sx + 1];
    dp[0] = true;

    let mut s = 0;
    for w in x {
        s += w;
        for i in (w..(s + 1)).rev() {
            dp[i] |= dp[i - w];
        }
    }
    let mut ret = Vec::new();
    for i in 0..sx {
        if dp[i + 1] {
            ret.push(i + 1)
        }
    }
    writeln!(cout, "{}", ret.len()).ok();
    for x in ret {
        write!(cout, "{} ", x).ok();
    }
    writeln!(cout, "").ok();
}
