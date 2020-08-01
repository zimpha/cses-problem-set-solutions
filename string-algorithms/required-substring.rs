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

    fn next_line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).expect("Failed to read line");
        line.trim().to_string()
    }
}

const MOD: u64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n: usize = cin.next();
    let s = cin.next_line();
    let s = s.as_bytes();
    let m = s.len();
    let border = border(s);
    let mut go = vec![[0; 26]; m];

    for i in 0..m {
        for o in 0..26 {
            let c = (o + 65) as u8;
            let mut j = i;
            while j > 0 && s[j] != c {
                j = border[j - 1];
            }
            if s[j] == c {
                j += 1;
            }
            go[i][o] = j;
        }
    }

    let mut f = vec![0; m];
    let mut ret = 0;
    f[0] = 1;

    for _ in 0..n {
        ret = ret * 26 % MOD;
        let mut g = vec![0; m];
        for j in 0..m {
            for o in 0..26 {
                let u = go[j][o];
                if u == m {
                    ret += f[j];
                } else {
                    g[u] += f[j];
                }
            }
        }
        ret %= MOD;
        for i in 0..m {
            f[i] = g[i] % MOD;
        }
    }

    writeln!(cout, "{}", ret).ok();
}

fn border<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut border = vec![0; n];
    for i in 1..n {
        let mut j = border[i - 1];
        while j > 0 && s[j] != s[i] {
            j = border[j - 1];
        }
        border[i] = if s[j] == s[i] { j + 1 } else { j }
    }
    border
}
