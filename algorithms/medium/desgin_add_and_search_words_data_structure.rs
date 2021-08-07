use std::collections::HashMap;

#[derive(Debug)]
struct Word {
    keys: Box<HashMap<u8, Box<Word>>>,
    is_end: bool,
}

impl Word {
    fn new() -> Self {
       Word {
           keys: Box::new(HashMap::new()),
           is_end: false,
        }   
    }    
}

struct WordDictionary {
    root: Word,
}

fn moving<T> (t: T) -> T { t }

fn rec(mut w: &Word, word: &[u8]) -> bool {    
    for i in 0..word.len() {
        if word[i] == b'.' {
            let subwords = &word[i+1..];
            for child in w.keys.values() {
                if rec(child, subwords) {
                    return true;
                }
            }
            return false;
        } else if !w.keys.contains_key(&word[i]) {
            return false;
        }
        
        w = w.keys.get(&word[i]).unwrap();
    }
    
    w.is_end
}

/** You can modify the type of `self` for your need. */
impl WordDictionary {


    /** Initialize your data structure here. */
    fn new() -> Self {
        WordDictionary {
            root: Word::new(),
        }
    }
    
    /** Adds a word into the data structure. */

    fn add_word(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.bytes() {
            curr = moving(curr).keys.entry(c).or_insert(Box::new(Word::new()));
        }
        curr.is_end = true;
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */

    fn search(&self, word: String) -> bool {
        rec(&self.root, word.as_bytes())
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */


