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

const MOD: u32 = 1_000_000_000 + 7;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n: usize = cin.next();
    let s = n * (n + 1) / 2;
    if s % 2 == 0 {
        let mut dp = vec![0; s / 2 + 1];
        dp[0] = 1;
        for x in 1..n + 1 {
            for i in (x..s / 2 + 1).rev() {
                dp[i] += dp[i - x];
                if dp[i] >= MOD {
                    dp[i] -= MOD;
                }
            }
        }
        writeln!(cout, "{}", dp[s / 2] as u64 * 500_000_004u64 % (MOD as u64)).ok();
    } else {
        writeln!(cout, "0").ok();
    }
}
