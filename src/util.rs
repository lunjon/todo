use crate::error::Result;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn random_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub fn try_get_env(name: &str) -> Option<String> {
    match std::env::var(name) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

pub fn read_file(path: &Path) -> Result<String> {
    let mut buf = String::new();
    let mut file = fs::OpenOptions::new().read(true).open(path)?;
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
pub fn word_chunks(s: &str, size: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for line in s.lines() {
        for l in line_chunks(line, size) {
            lines.push(l.to_string());
        }
    }
    lines
}

fn line_chunks(s: &str, size: usize) -> Vec<String> {
    if s.len() < size {
        return vec![s.to_string()];
    }

    let mut words: Vec<String> = s.split_whitespace().map(String::from).collect();
    words.reverse();
    let mut chunks = Vec::new();

    loop {
        let mut done = false;
        let mut chunk = String::new();

        loop {
            let word = match words.pop() {
                Some(word) => word,
                None => {
                    done = true;
                    break;
                }
            };

            if chunk.is_empty() && word.len() > size {
                // Current word is longer than size, split word
                let left = size - (chunk.len() + 1);
                chunk.push_str(&word[0..left - 1]);
                chunk.push('-');
                words.push(word[left - 1..].to_string());
            } else if chunk.len() + word.len() <= size {
                // Next word fits in this chunk
                if chunk.len() + word.len() < size {
                    chunk.push_str(&word);
                    chunk.push(' ');
                } else {
                    chunk.push_str(&word);
                }
            } else {
                // Chunk is full
                words.push(word);
                break;
            }
        }

        chunks.push(chunk);
        if done {
            break;
        }
    }

    chunks
}

#[test]
fn test_word_chunks() {
    let s = "one two three four"; // length == 18
    let a = word_chunks(s, 10);
    assert_eq!(a.len(), 2);

    let b = word_chunks(s, 5);
    dbg!(&b);
    assert_eq!(b.len(), 4);
}
