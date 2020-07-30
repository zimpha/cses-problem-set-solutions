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
    let k: usize = cin.next();
    let a: Vec<u64> = cin.next_vec_n(n);
    let mut left = *a.iter().max().unwrap();
    let mut right: u64 = a.iter().sum();

    while left < right {
        let mid = (left + right - 1) >> 1;
        let mut i = 0;
        let mut cnt = 0;
        while i < n {
            let mut sum = 0;
            let mut j = i;
            while j < n && sum + a[j] <= mid {
                sum += a[j];
                j += 1;
            }
            i = j;
            cnt += 1;
        }
        if cnt <= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    writeln!(cout, "{}", left).ok();
}
