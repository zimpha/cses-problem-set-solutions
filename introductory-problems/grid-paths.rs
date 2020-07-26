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

struct Solution {
    path: Vec<char>,
    mark: Vec<Vec<bool>>,
    ways: u64,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            mark: vec![vec![false; 7]; 7],
            ways: 0,
        }
    }

    fn run(&mut self) {
        let (stdin, stdout) = (io::stdin(), io::stdout());
        let mut cin = Scanner::new(stdin.lock());
        let mut cout = io::BufWriter::new(stdout.lock());

        self.path = cin.next_line().chars().collect();
        self.mark[0][0] = true;
        self.dfs(0, 0, 0);

        writeln!(cout, "{}", self.ways).ok();
    }

    fn dfs(&mut self, s: usize, x: usize, y: usize) {
        if x == 6 && y == 0 {
            if s == 48 {
                self.ways += 1;
            }
            return;
        }
        // hit a wall or path, both left and right can go
        if (x == 6 || x == 0 || (self.mark[x + 1][y] && self.mark[x - 1][y]))
            && y > 0
            && y < 6
            && !self.mark[x][y - 1]
            && !self.mark[x][y + 1]
        {
            return;
        }
        // hit a wall or path, both left and right can go
        if (y == 6 || y == 0 || (self.mark[x][y - 1] && self.mark[x][y + 1]))
            && x > 0
            && x < 6
            && !self.mark[x - 1][y]
            && !self.mark[x + 1][y]
        {
            return;
        }
        self.mark[x][y] = true;
        if self.path[s] == 'U' && x > 0 && !self.mark[x - 1][y] {
            self.dfs(s + 1, x - 1, y);
        } else if self.path[s] == 'D' && x < 6 && !self.mark[x + 1][y] {
            self.dfs(s + 1, x + 1, y);
        } else if self.path[s] == 'L' && y > 0 && !self.mark[x][y - 1] {
            self.dfs(s + 1, x, y - 1);
        } else if self.path[s] == 'R' && y < 6 && !self.mark[x][y + 1] {
            self.dfs(s + 1, x, y + 1);
        } else if self.path[s] == '?' {
            if x > 0 && !self.mark[x - 1][y] {
                self.dfs(s + 1, x - 1, y);
            }
            if x < 6 && !self.mark[x + 1][y] {
                self.dfs(s + 1, x + 1, y);
            }
            if y > 0 && !self.mark[x][y - 1] {
                self.dfs(s + 1, x, y - 1);
            }
            if y < 6 && !self.mark[x][y + 1] {
                self.dfs(s + 1, x, y + 1);
            }
        }
        self.mark[x][y] = false;
    }
}

fn main() {
    let mut sol = Solution::new();
    sol.run()
}
