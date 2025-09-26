use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    let ss : String= words.to_ascii_lowercase().chars().map(|c| if c.is_alphanumeric() || c == '\'' {c} else {' '}).collect();
     
    for mut v in ss.split_whitespace()  {

        v = v.trim_matches('\'');
        *map.entry(v.to_string()).or_insert(0)+=1;
      
    }
    map

}