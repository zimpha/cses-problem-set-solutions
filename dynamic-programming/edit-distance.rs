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
        if line.ends_with('\n') {
            line.pop();
        }
        line
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
    let mut dp = vec![vec![n * m; m + 1]; n + 1];

    for i in 0..n {
        dp[i + 1][0] = i + 1;
    }
    for i in 0..m {
        dp[0][i + 1] = i + 1;
    }
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = min(dp[i][j + 1], dp[i + 1][j]) + 1;
            dp[i + 1][j + 1] = min(dp[i + 1][j + 1], dp[i][j] + 1);
            if s[i] == t[j] {
                dp[i + 1][j + 1] = min(dp[i][j], dp[i + 1][j + 1]);
            }
        }
    }
    writeln!(cout, "{}", dp[n][m]).ok();
}
