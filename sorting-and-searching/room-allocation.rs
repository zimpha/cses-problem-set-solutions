use std::{
    collections::BTreeSet,
    io::{self, Write},
    ops::Bound::Excluded,
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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n: usize = cin.next();
    let mut iv = Vec::with_capacity(n);
    for i in 0..n {
        let a: u32 = cin.next();
        let b: u32 = cin.next();
        iv.push((b, a, i));
    }
    iv.sort();

    let mut ret = vec![0; n];
    let mut room: BTreeSet<(u32, usize)> = BTreeSet::new();

    for (ed, st, i) in iv {
        if let Some(&entry) = room.range((Excluded(&(0, 0)), Excluded(&(st, 0)))).last() {
            room.remove(&entry);
            room.insert((ed, entry.1));
            ret[i] = entry.1;
        } else {
            ret[i] = room.len();
            room.insert((ed, room.len()));
        }
    }

    writeln!(cout, "{}", room.len()).ok();
    for x in ret {
        write!(cout, "{} ", x + 1).ok();
    }
    writeln!(cout, "").ok();
}
