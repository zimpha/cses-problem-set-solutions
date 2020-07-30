use std::{
    collections::BTreeMap,
    io::{self, Write},
    ops::Bound::Included,
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
    let m: usize = cin.next();
    let h: Vec<u32> = cin.next_vec_n(n);
    let t: Vec<u32> = cin.next_vec_n(m);

    let mut price = BTreeMap::new();
    for p in h {
        *price.entry(p).or_insert(0) += 1;
    }

    for max_price in t {
        let range = price.range_mut((Included(&1), Included(&max_price)));
        let mut remove = None;
        if let Some((p, c)) = range.last() {
            *c -= 1;
            if *c == 0 {
                remove = Some(*p);
            }
            writeln!(cout, "{}", p).ok();
        } else {
            writeln!(cout, "{}", -1).ok();
        }
        if let Some(p) = remove {
            price.remove(&p);
        }
    }
}
