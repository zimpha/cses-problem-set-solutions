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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let t = cin.next::<u32>();
    for _ in 0..t {
        let x = cin.next::<u64>();
        let y = cin.next::<u64>();
        let ret;
        if x == y {
            ret = x * x + 1 - x;
        } else if x > y {
            if x % 2 == 0 {
                ret = x * x + 1 - y;
            } else {
                ret = (x - 1) * (x - 1) + y;
            }
        } else {
            if y % 2 == 1 {
                ret = y * y + 1 - x;
            } else {
                ret = (y - 1) * (y - 1) + x;
            }
        }
        writeln!(cout, "{}", ret).ok();
    }
}
