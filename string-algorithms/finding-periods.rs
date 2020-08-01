use std::io::{self, Write};

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
    let n = s.len();
    let border = border(s.as_bytes());

    let mut j = n;
    let mut ret = Vec::new();
    loop {
        j = border[j - 1];
        ret.push(j);
        if j == 0 {
            break;
        }
    }
    for b in ret {
        write!(cout, "{} ", n - b).ok();
    }
    writeln!(cout, "").ok();
}

fn border<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut border = vec![0; n];
    for i in 1..n {
        let mut j = border[i - 1];
        while j > 0 && s[j] != s[i] {
            j = border[j - 1];
        }
        border[i] = if s[j] == s[i] { j + 1 } else { j }
    }
    border
}
