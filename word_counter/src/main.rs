use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    println!("Enter file path:");
    let x = get_input();
    let path = Path::new(&x);

    let content: String = read_file_content(path);

    let lines = count_lines(&content);
    let words = content.split_whitespace().collect::<Vec<&str>>();
    let chars = content.chars().collect::<Vec<char>>().len();

    let mut top_words: HashMap<String, usize> = HashMap::new();

    for word in &words {
        let word = word.to_lowercase();
        *top_words.entry(word).or_insert(0) += 1;
    }

    let mut top_vec: Vec<(String, usize)> = top_words.into_iter().collect();
    top_vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Lines: {}", lines);
    println!("Words: {}", words.len());
    println!("Chars: {}", chars);

    println!("Map len: {}", top_words.len());
    println!("\n");

    println!("Top 5 words:");
    for (word, count) in top_vec.iter().take(5) {
        println!("{}: {}", word, count);
    }
}

fn get_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn read_file_content(path: &Path) -> String {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            println!("err: {}", err);
            panic!("Failed!!")
        }
    };
    content
}

fn count_lines(content: &str) -> usize {
    content.lines().count()
}
