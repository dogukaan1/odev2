use std::io;




fn main() {

    println!("cümleyi giriniz: ");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("hata");


    let word_counter = WordCounter::new(&input_text);

   
    let word_count = word_counter.count_words();
    println!("kelime sayısı: {}", word_count);
}
struct WordCounter {
    text: String,
}


impl WordCounter {
   
    fn new(text: &str) -> WordCounter {
        WordCounter { text: String::from(text) }
    }

 
    fn count_words(&self) -> usize {
        let words: Vec<&str> = self.text.split_whitespace().collect();
        words.len()
    }
}