use std::{
    cmp::min,
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
    let m: usize = cin.next();
    let mut dp = vec![vec![n * m; m]; n];
    for i in 0..n {
        for j in 0..m {
            if i == j {
                dp[i][j] = 0;
            } else {
                let mut best = n * m;
                for k in 1..i + 1 {
                    best = min(best, dp[k - 1][j] + dp[i - k][j]);
                }
                for k in 1..j + 1 {
                    best = min(best, dp[i][k - 1] + dp[i][j - k]);
                }
                dp[i][j] = best + 1;
            }
        }
    }
    writeln!(cout, "{}", dp[n - 1][m - 1]).ok();
}
