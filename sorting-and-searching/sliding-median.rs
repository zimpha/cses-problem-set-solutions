use std::{
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

    let n: usize = cin.next();
    let m: usize = cin.next();
    let a: Vec<u32> = cin.next_vec_n(n);
    let mm = (m + 1) / 2;

    let mut spq = BTreeSet::new();
    let mut lpq = BTreeSet::new();

    for i in 0..m - 1 {
        add(&mut spq, &mut lpq, (a[i], i), mm);
    }
    for i in m - 1..n {
        add(&mut spq, &mut lpq, (a[i], i), mm);
        //println!("+ {:?} {:?}", spq, lpq);
        write!(cout, "{} ", spq.range(..).last().unwrap().0).ok();
        let rm = (a[i + 1 - m], i + 1 - m);
        if spq.contains(&rm) {
            spq.remove(&rm);
            if !lpq.is_empty() {
                let w = *lpq.range(..).next().unwrap();
                lpq.remove(&w);
                spq.insert(w);
            }
        } else {
            lpq.remove(&rm);
        }
        //println!("- {:?} {:?}", spq, lpq);
    }
    writeln!(cout, "").ok();
}

fn add(spq: &mut BTreeSet<(u32, usize)>, lpq: &mut BTreeSet<(u32, usize)>, val: (u32, usize), bound: usize) {
    spq.insert(val);
    if spq.len() <= bound {
        return;
    }
    while spq.len() > bound {
        let last = *spq.range(..).last().unwrap();
        lpq.insert(last);
        spq.remove(&last);
    }
    let x = *spq.range(..).last().unwrap();
    let y = *lpq.range(..).next().unwrap();
    if x.0 > y.0 {
        spq.remove(&x);
        lpq.remove(&y);
        spq.insert(y);
        lpq.insert(x);
    }
}
