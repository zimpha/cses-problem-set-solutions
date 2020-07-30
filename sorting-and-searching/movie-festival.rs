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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n: usize = cin.next();
    let mut movies = Vec::new();
    for _ in 0..n {
        let a: u32 = cin.next();
        let b: u32 = cin.next();
        movies.push((b, a));
    }

    movies.sort();
    let mut dp = vec![0; n];
    for i in 0..n {
        let st = movies[i].1;
        let index = movies.binary_search(&(st, st)).unwrap_or_else(|x| x);
        if index > 0 {
            dp[i] = dp[index - 1] + 1;
        } else {
            dp[i] = 1;
        }
        if i > 0 && dp[i] < dp[i - 1] {
            dp[i] = dp[i - 1];
        }
    }
    writeln!(cout, "{}", dp[n - 1]).ok();
}
