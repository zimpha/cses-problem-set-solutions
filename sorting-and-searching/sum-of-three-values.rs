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
    let x: u32 = cin.next();
    let a: Vec<u32> = cin.next_vec_n(n);
    let mut ai = Vec::with_capacity(n);

    for i in 0..n {
        ai.push((a[i], i + 1));
    }

    ai.sort();

    let mut found = false;

    for i in 0..n {
        if found || ai[i].0 >= x {
            break;
        }
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            while j < k && ai[i].0 + ai[j].0 + ai[k].0 > x {
                k -= 1;
            }
            if j < k && ai[i].0 + ai[j].0 + ai[k].0 == x {
                writeln!(cout, "{} {} {}", ai[i].1, ai[j].1, ai[k].1).ok();
                found = true;
                break;
            }
            j += 1;
        }
    }

    if !found {
        writeln!(cout, "IMPOSSIBLE").ok();
    }
}
