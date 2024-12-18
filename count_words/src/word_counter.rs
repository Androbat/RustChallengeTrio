use std::collections::HashMap;
use std::fs::File;
use std::io::{ self, BufRead, BufReader};

#[derive(Debug)]
pub struct FileProcessor {
    file: File,
}

impl FileProcessor {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(FileProcessor { file })
    }

    pub fn count_words(&mut self) -> io::Result<HashMap<String, i32>>{
        let mut word_count = HashMap::new();
        let reader = BufReader::new(&self.file);
        
        
        for line in reader.lines(){
            let line = line?;
            let words = line.split_whitespace();
            
            for word in words {
                // Lower case any character in order to not count different words
                // then replace every spacial chacters with an empty &str in order to not count it as a word.
                let clean_words = word.to_lowercase().replace(|c: char| !c.is_alphabetic(), "");
                if !clean_words.is_empty(){
                    *word_count.entry(clean_words).or_insert(0) += 1;
                }
            }

        }
        
        Ok(word_count)
    }

}