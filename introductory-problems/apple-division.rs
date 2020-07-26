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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n = cin.next::<usize>();
    let v = cin.next_vec_n::<u64>(n);
    let sum: u64 = v.iter().sum();
    let mut ret = sum;
    for mask in 0..(1 << n) {
        let mut now: u64 = 0;
        for i in 0..n {
            if (mask >> i & 1) == 1 {
                now += v[i];
            }
        }
        if sum >= now * 2 {
            ret = std::cmp::min(ret, sum - now * 2);
        } else {
            ret = std::cmp::min(ret, now * 2 - sum);
        }
    }
    writeln!(cout, "{}", ret).ok();
}
