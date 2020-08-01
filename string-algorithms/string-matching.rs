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

    let s: Vec<char> = cin.next_line().chars().collect();
    let t: Vec<char> = cin.next_line().chars().collect();
    let n = s.len();
    let m = t.len();
    let mut border = vec![0; m];

    for i in 1..m {
        let mut j = border[i - 1];
        while j > 0 && t[j] != t[i] {
            j = border[j - 1];
        }
        border[i] = if t[j] == t[i] { j + 1 } else { j }
    }

    let mut ret = 0;
    let mut j = 0;
    for i in 0..n {
        while j > 0 && t[j] != s[i] {
            j = border[j - 1];
        }

        if t[j] == s[i] {
            j += 1
        }
        if j == m {
            ret += 1;
            j = border[j - 1];
        }
    }

    writeln!(cout, "{}", ret).ok();
}
