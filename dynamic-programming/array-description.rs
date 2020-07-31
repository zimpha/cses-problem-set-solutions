use std::{
    io::{self, Write},
    mem, str,
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

const MOD: u32 = 1_000_000_000 + 7;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n: usize = cin.next();
    let m: usize = cin.next();
    let x: Vec<usize> = cin.next_vec_n(n);
    let mut f = vec![0; m];
    let mut g;

    if x[0] == 0 {
        f = vec![1; m];
    } else {
        f[x[0] - 1] = 1;
    }

    for &v in &x[1..] {
        g = vec![0; m];
        let (l, r) = if v == 0 { (0, m) } else { (v - 1, v) };

        for i in l..r {
            g[i] = f[i];
            if i > 0 {
                g[i] += f[i - 1];
            }
            if i + 1 < m {
                g[i] += f[i + 1];
            }
            g[i] %= MOD;
        }
        mem::swap(&mut f, &mut g);
    }
    let mut ret = 0;
    for x in f {
        ret = (ret + x) % MOD;
    }
    writeln!(cout, "{}", ret % MOD).ok();
}
