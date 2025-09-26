pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
   
 let k = letters_per_turn as usize;

 if k == 0 || s.is_empty(){
    return None;
 }
 let mut tab = vec![String::new();k];

 for (i,v) in s.chars().enumerate() {
    tab[i%k].push(v);
 }

 Some(tab.concat())

}
//  scytale_decoder("aebfcgd".to_string(), 2),
//Some("abcdefg".to_string())
//oenset  daa yt hirne et hfea lflosr
//one day in the forest a three falls


/*
"sec yCtoadle" size=2 -> Some("scytale Code")
"steoca dylCe" size=4 -> Some("scytale Code")

Example

letters_per_turn 2: "scytale Code" -> "sec yCtoadle"    scytale Code

--------------------------------
  |s|  |c|  |y|  |t|  |a|  |l|
  |e|  | |  |C|  |o|  |d|  |e|
--------------------------------
sec yC
toadle
letters_per_turn 4: "scytale Code" -> "steoca dylCe"  scytale Code

------------------------------------------
  |s|  |c|  |y|
  |t|  |a|  |l|
  |e|  | |  |C|
  |o|  |d|  |e|
------------------------------------------

*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_args() {
        assert_eq!(scytale_decoder("".to_string(), 5), None);
        assert_eq!(scytale_decoder("empty test".to_string(), 0), None);
        assert_eq!(scytale_decoder("".to_string(), 0), None);
    }

    #[test]
    fn test_short_nb_letters() {
        assert_eq!(
            scytale_decoder("This is already decoded".to_string(), 100),
            Some("This is already decoded".to_string())
        );
    }

    #[test]
    fn test_short_sentence() {
        assert_eq!(
            scytale_decoder("aebfcgd".to_string(), 2),
            Some("abcdefg".to_string())
        );
    }

    #[test]
    fn test_medium_sentence() {
        assert_eq!(
            scytale_decoder("oenset  daa yt hirne et hfea lflosr".to_string(), 2),
            Some("one day in the forest a three falls".to_string())
        );
    }

    #[test]
    fn test_long_sentence() {
        assert_eq!(
            scytale_decoder(
                "dbtheouoevyigleolepnudtmmwhheaaoegnnurigtsavoteneeosdss".to_string(),
                5
            ),
            Some("doyouwanttobuildhousestogetherandhelpmegivesevenmangoes".to_string())
        );
    }
}
