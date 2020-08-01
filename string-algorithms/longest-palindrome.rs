use std::{
    cmp::min,
    io::{self, Write},
};

struct Scanner<R> {
    reader: R,
}

impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self { reader }
    }

    fn next_line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).expect("Failed to read line");
        line.trim().to_string()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let s = cin.next_line();
    let d = manacher(s.as_bytes());

    let mut best = 1;
    let mut best_idx = 0;
    for i in 1..d.len() {
        if d[i] > best {
            best = d[i];
            best_idx = i;
        }
    }

    let (l, r) = if best_idx % 2 == 0 {
        (best_idx / 2 - best / 2, best_idx / 2 + best / 2)
    } else {
        (best_idx / 2 + 1 - best / 2, best_idx / 2 + best / 2)
    };
    writeln!(cout, "{}", s.get(l..r + 1).unwrap()).ok();
}

fn manacher<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len() * 2;
    let mut d = vec![0; n];
    let mut i = 0;
    let mut j = 0;
    while i < n {
        while i >= j && i + j + 1 < n && s[(i - j) >> 1] == s[(i + j + 1) >> 1] {
            j += 1;
        }
        d[i] = j;
        let mut k = 1;
        while i >= k && d[i] >= k && d[i - k] != d[i] - k {
            d[i + k] = min(d[i - k], d[i] - k);
            k += 1;
        }
        i += k;
        j = if j >= k { j - k } else { 0 };
    }
    d
}
