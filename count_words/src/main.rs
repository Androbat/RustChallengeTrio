use std::io;

mod word_counter;
fn main() -> io::Result<()> {
    let mut words = match word_counter::FileProcessor::new("../book.txt"){
        Ok(proccesed_file) => proccesed_file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e);
        }
    };
    let count = words.count_words()?;
    println!("{:?}", count);
    Ok(())
}
