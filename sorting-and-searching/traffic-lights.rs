use std::{
    cmp::max,
    collections::BTreeSet,
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

    let x: u32 = cin.next();
    let n: usize = cin.next();
    let p: Vec<u32> = cin.next_vec_n(n);
    let mut xs = p.clone();
    xs.push(0);
    xs.push(x);
    xs.sort();

    let mut best = 0;
    for i in 0..n + 1 {
        best = max(best, xs[i + 1] - xs[i]);
    }
    let mut pos: BTreeSet<u32> = xs.into_iter().collect();
    let mut ret = Vec::with_capacity(n);
    for x in p.iter().rev() {
        ret.push(best);
        pos.remove(x);
        let r = pos.range(x..).next().unwrap();
        let l = pos.range(..x).last().unwrap();
        best = max(best, r - l);
    }
    ret.reverse();

    for x in ret {
        write!(cout, "{} ", x).ok();
    }
    writeln!(cout, "").ok();
}
