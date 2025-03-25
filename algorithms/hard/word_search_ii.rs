#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            node = node.children[(b - b'a') as usize].get_or_insert(Default::default());
        }
        node.is_end = true;
    }
}

fn dfs(
    board: &mut Vec<Vec<char>>,
    trie: &mut Trie,
    i: usize,
    j: usize,
    curr: &mut String,
    ans: &mut Vec<String>,
) {
    let c = board[i][j];
    if let Some(node) = &mut trie.children[(c as u8 - b'a') as usize] {
        curr.push(c);
        if node.is_end {
            ans.push(curr.to_string());
            node.is_end = false;
        }

        board[i][j] = '#';
        for (i, j) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
            if i < board.len() && j < board[i].len() && board[i][j] != '#' {
                dfs(board, node, i, j, curr, ans);
            }
        }
        board[i][j] = c;
        curr.pop();
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::default();
        for w in words {
            trie.insert(w);
        }

        let mut ans = Vec::new();

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                dfs(&mut board, &mut trie, i, j, &mut String::new(), &mut ans);
            }
        }

        ans
    }
}
