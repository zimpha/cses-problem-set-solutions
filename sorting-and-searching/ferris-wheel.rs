use std::{
    collections::BinaryHeap,
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
    let mut a: Vec<u32> = cin.next_vec_n(n);
    a.sort();
    a.reverse();

    let mut pq = BinaryHeap::new();
    let mut ret = 0;
    for i in 0..n {
        if i == a.len() {
            break;
        }
        let v = a[i];
        while a.len() > i && a.last().unwrap() + v <= x {
            let u = a.pop().unwrap();
            pq.push(u);
        }
        if a.len() == i {
            break;
        }
        ret += 1;
        if !pq.is_empty() {
            pq.pop();
        }
    }
    ret += (pq.len() + 1) / 2;

    writeln!(cout, "{}", ret).ok();
}
