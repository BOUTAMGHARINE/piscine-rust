use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
  let mut res : HashMap<String,u32>= HashMap :: new();

  for word in words.split_whitespace() {
     *res.entry(word.to_string().to_uppercase()).or_insert(0) += 1;
  }
    
res


}
