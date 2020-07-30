use std::{
    cmp::max,
    collections::VecDeque,
    i64,
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
    let l: usize = cin.next();
    let r: usize = cin.next();
    let a: Vec<i64> = cin.next_vec_n(n);

    let mut sum = vec![0; n + 1];
    let mut queue = VecDeque::new();
    let mut ret = i64::MIN;

    for i in 1..n + 1 {
        while i >= r && !queue.is_empty() && *queue.front().unwrap() < i - r {
            queue.pop_front();
        }
        if i >= l {
            while !queue.is_empty() && sum[*queue.back().unwrap()] >= sum[i - l] {
                queue.pop_back();
            }
            queue.push_back(i - l);
        }
        sum[i] = sum[i - 1] + a[i - 1];
        if !queue.is_empty() {
            ret = max(ret, sum[i] - sum[*queue.front().unwrap()]);
        }
    }

    writeln!(cout, "{}", ret).ok();
}
