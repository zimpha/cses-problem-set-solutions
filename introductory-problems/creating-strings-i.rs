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
        if line.ends_with('\n') {
            line.pop();
        }
        line
    }
}

fn dfs(result: &mut Vec<String>, depth: usize, cnt: &mut Vec<usize>, s: &mut Vec<u8>) {
    if depth == s.len() {
        result.push(String::from(std::str::from_utf8(s).unwrap()));
        return;
    }
    for c in 0..26 {
        if cnt[c] > 0 {
            cnt[c] -= 1;
            s[depth] = c as u8 + 97;
            dfs(result, depth + 1, cnt, s);
            cnt[c] += 1;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let mut s = cin.next_line();
    let mut cnt = vec![0; 26];
    for c in s.as_bytes() {
        cnt[(c - 97) as usize] += 1;
    }
    unsafe {
        let s = s.as_mut_vec();
        let mut result = Vec::new();
        dfs(&mut result, 0, &mut cnt, s);
        writeln!(cout, "{}", result.len()).ok();
        for s in result {
            writeln!(cout, "{}", s).ok();
        }
    }
}
