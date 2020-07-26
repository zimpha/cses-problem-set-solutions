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
    grid: Vec<Vec<char>>,
    mark_col: Vec<bool>,
    mark_main_dia: Vec<bool>,
    mark_anti_dia: Vec<bool>,
    ways: u64,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(),
            mark_col: vec![false; 8],
            mark_main_dia: vec![false; 16],
            mark_anti_dia: vec![false; 16],
            ways: 0,
        }
    }

    fn run(&mut self) {
        let (stdin, stdout) = (io::stdin(), io::stdout());
        let mut cin = Scanner::new(stdin.lock());
        let mut cout = io::BufWriter::new(stdout.lock());
        for _ in 0..8 {
            self.grid.push(cin.next_line().chars().collect());
        }

        self.dfs(0);

        writeln!(cout, "{}", self.ways).ok();
    }

    fn dfs(&mut self, row: usize) {
        if row == 8 {
            self.ways += 1;
            return;
        }
        for col in 0..8 {
            if !self.mark_col[col]
                && !self.mark_main_dia[row + col]
                && !self.mark_anti_dia[row + 8 - col]
                && self.grid[row][col] == '.'
            {
                self.mark_col[col] = true;
                self.mark_main_dia[row + col] = true;
                self.mark_anti_dia[row + 8 - col] = true;
                self.dfs(row + 1);
                self.mark_col[col] = false;
                self.mark_main_dia[row + col] = false;
                self.mark_anti_dia[row + 8 - col] = false;
            }
        }
    }
}

fn main() {
    let mut sol = Solution::new();
    sol.run()
}
