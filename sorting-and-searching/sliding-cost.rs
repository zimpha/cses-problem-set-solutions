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
    let a: Vec<u64> = cin.next_vec_n(n);
    let mm = (m + 1) / 2;

    let mut spq = BTreeSet::new();
    let mut lpq = BTreeSet::new();
    let mut ssum = 0;
    let mut lsum = 0;

    for i in 0..m - 1 {
        add(&mut spq, &mut ssum, &mut lpq, &mut lsum, (a[i], i), mm);
    }
    for i in m - 1..n {
        add(&mut spq, &mut ssum, &mut lpq, &mut lsum, (a[i], i), mm);
        let med = spq.range(..).last().unwrap().0;
        write!(
            cout,
            "{} ",
            med * spq.len() as u64 - ssum + lsum - med * lpq.len() as u64
        )
        .ok();
        let rm = (a[i + 1 - m], i + 1 - m);
        if spq.contains(&rm) {
            ssum -= rm.0;
            spq.remove(&rm);
            if !lpq.is_empty() {
                let w = *lpq.range(..).next().unwrap();
                lsum -= w.0;
                ssum += w.0;
                lpq.remove(&w);
                spq.insert(w);
            }
        } else {
            lsum -= rm.0;
            lpq.remove(&rm);
        }
    }
    writeln!(cout, "").ok();
}

fn add(
    spq: &mut BTreeSet<(u64, usize)>,
    ssum: &mut u64,
    lpq: &mut BTreeSet<(u64, usize)>,
    lsum: &mut u64,
    val: (u64, usize),
    bound: usize,
) {
    *ssum += val.0;
    spq.insert(val);
    if spq.len() <= bound {
        return;
    }
    while spq.len() > bound {
        let last = *spq.range(..).last().unwrap();
        *lsum += last.0;
        *ssum -= last.0;
        lpq.insert(last);
        spq.remove(&last);
    }
    let x = *spq.range(..).last().unwrap();
    let y = *lpq.range(..).next().unwrap();
    if x.0 > y.0 {
        *ssum -= x.0;
        *lsum -= y.0;
        spq.remove(&x);
        lpq.remove(&y);
        *ssum -= y.0;
        *lsum -= x.0;
        spq.insert(y);
        lpq.insert(x);
    }
}
