use std::collections::HashMap;
use regex::Regex;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();
    
    // Expression régulière pour conserver les lettres, chiffres et apostrophes internes aux mots
    let re = Regex::new(r"[^a-zA-Z0-9']+").unwrap();  // Supprime tout sauf lettres, chiffres et apostrophes

    for word in words.split_whitespace() {
        // Nettoie chaque mot
        let cleaned_word = if word.starts_with('\'') && word.ends_with('\'') {
            // Conserve les apostrophes autour des mots
            word.to_string()  // On garde le mot tel quel avec les apostrophes
        } else {
            // Sinon, on nettoie les ponctuations tout en gardant les apostrophes à l'intérieur
            re.replace_all(word, "").to_string()
        };

        // Met le mot en minuscules pour éviter de différencier "Joe" de "joe"
        let cleaned_word = cleaned_word.to_lowercase();

        // Ne pas ajouter de mots vides
        if !cleaned_word.is_empty() {
            *res.entry(cleaned_word).or_insert(0)+=1;
        }
    }
    
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_counting(input: &str, expected: &[(&str, u32)]) {
        let mut m: HashMap<String, u32> = counting_words(input);
        for &(k, v) in expected.iter() {
            assert_eq!(
                (k, m.remove(&k.to_string()).unwrap_or(0)),
                (k, v)
            );
        }
        // Vérifie qu'il n'y a pas d'éléments restants dans la HashMap
        assert_eq!(m.iter().collect::<Vec<(&String, &u32)>>(), vec![]);
    }

    #[test]
    fn test_apostrophe() {
        test_counting(
            "Joe can't tell between 'large' and large.",
            &[
                ("joe", 1),
                ("can't", 1),
                ("tell", 1),
                ("between", 1),
                ("'large'", 1),  // 'large' doit être compté avec les apostrophes
                ("large", 1),
                ("and", 1),
            ],
        );
    }
}
