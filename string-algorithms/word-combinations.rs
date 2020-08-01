use std::{
    io::{self, Write},
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

    fn next_line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).expect("Failed to read line");
        line.trim().to_string()
    }
}

const MOD: u32 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let s: Vec<char> = cin.next_line().chars().collect();
    let mut ac = Trie::with_capacity(1_000_000 + 10);
    let m: usize = cin.next();
    let n = s.len();

    ac.init();
    for _ in 0..m {
        let t = cin.next_line();
        ac.insert(&t);
    }

    let mut dp = vec![0; n + 1];

    dp[n] = 1;
    for i in (0..n).rev() {
        let mut p = ac.root;
        for j in i..n {
            let o = s[j] as usize - 97;
            p = ac.arena[p].go[o];
            if p == NULL_INDEX {
                break;
            }
            if ac.arena[p].is_leaf {
                dp[i] += dp[j + 1];
                if dp[i] >= MOD {
                    dp[i] -= MOD;
                }
            }
        }
    }

    writeln!(cout, "{}", dp[0]).ok();
}

const NULL_INDEX: usize = !0;

#[derive(Clone)]
struct TrieNode {
    pub go: [usize; 26],
    pub is_leaf: bool,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            go: [NULL_INDEX; 26],
            is_leaf: false,
        }
    }
}

struct Trie {
    pub arena: Vec<TrieNode>,
    pub root: usize,
    pub size: usize,
}

impl Trie {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: vec![Default::default(); capacity],
            root: NULL_INDEX,
            size: 0,
        }
    }

    pub fn init(&mut self) {
        self.size = 0;
        self.root = self.alloc();
    }

    pub fn insert(&mut self, s: &str) {
        let mut p = self.root;
        for c in s.chars() {
            let o = c as usize - 97;
            if self.arena[p].go[o] == NULL_INDEX {
                self.arena[p].go[o] = self.alloc();
            }
            p = self.arena[p].go[o];
        }
        self.arena[p].is_leaf = true;
    }

    fn alloc(&mut self) -> usize {
        self.arena[self.size].go = [NULL_INDEX; 26];
        self.arena[self.size].is_leaf = false;
        self.size += 1;
        self.size - 1
    }
}
