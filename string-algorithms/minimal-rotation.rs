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

    let mut s: Vec<char> = cin.next_line().chars().collect();
    let idx = minimal_rotation(&s);
    s.rotate_left(idx);
    let s: String = s.into_iter().collect();
    writeln!(cout, "{}", s).ok();
}

fn minimal_rotation<T: Ord>(s: &[T]) -> usize {
    let n = s.len();
    let mut i = 0;
    let mut j = 1;
    loop {
        let mut k = 0;
        let mut ci = &s[i % n];
        let mut cj = &s[j % n];
        while k < n {
            ci = &s[(i + k) % n];
            cj = &s[(j + k) % n];
            if ci != cj {
                break;
            }
            k += 1
        }
        if k == n {
            return min(i, j);
        }
        if ci > cj {
            i += k + 1;
            i += (i == j) as usize;
        } else {
            j += k + 1;
            j += (i == j) as usize;
        }
    }
}
