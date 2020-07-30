use std::{
    collections::HashMap,
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
    let x: u32 = cin.next();

    let a: Vec<u32> = cin.next_vec_n(n);
    let mut ai = Vec::with_capacity(n);
    for i in 0..n {
        ai.push((a[i], i + 1));
    }
    ai.sort();

    let mut two_sum = HashMap::new();
    for i in 0..n {
        for j in i + 1..n {
            two_sum.entry(ai[i].0 + ai[j].0).or_insert(vec![]).push((i, j));
        }
    }

    let mut found = false;
    for (&k, v) in &two_sum {
        if k > x || !two_sum.contains_key(&(x - k)) {
            continue;
        }
        let u = two_sum.get(&(x - k)).unwrap();
        let mut bestl = v[0];
        let mut bestr = u[0];
        for e in v {
            if e.1 < bestl.1 {
                bestl = *e;
            }
        }
        for e in u {
            if e.0 > bestr.0 {
                bestr = *e;
            }
        }
        if bestl.1 < bestr.0 {
            writeln!(
                cout,
                "{} {} {} {}",
                ai[bestl.0].1, ai[bestl.1].1, ai[bestr.0].1, ai[bestr.1].1
            )
            .ok();
            found = true;
            break;
        }
    }

    if !found {
        writeln!(cout, "IMPOSSIBLE").ok();
    }
}
