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
    let a: Vec<i64> = cin.next_vec_n(n);
    let sum: i64 = a.iter().sum();
    let mut stk = Vec::new();
    let mut m = 0;
    
    for x in a {
        m += 1;
        stk.push(x);
        while m >= 3 && stk[m - 3] <= stk[m - 2] && stk[m - 2] >= stk[m - 1] {
            stk[m - 3] = stk[m - 3] - stk[m - 2] + stk[m - 1];
            stk.pop();
            stk.pop();
            m -= 2;
        }
    }

    let mut l = 0;
    let mut r = m;
    let mut delta = 0;
    for i in 0..m {
        let w = if stk[l] > stk[r - 1] {
            l += 1;
            stk[l - 1]
        } else {
            r -= 1;
            stk[r]
        };
        if i % 2 == 1 {
            delta -= w;
        } else {
            delta += w;
        }
    }

    writeln!(cout, "{}", (sum + delta) / 2).ok();
}
