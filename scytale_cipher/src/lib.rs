pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
   
    let len = s.len() as u32;
    if s.len()==0 || letters_per_turn == 0  {
        return None
    }else if letters_per_turn>= len{
        return Some(s)
    }
    let n = letters_per_turn;
    let chars: Vec<char> = s.chars().collect();
    let mut res = String::new();
    let mut count : u32= 0;
    
    res.push(chars[0]);
     while res.len() < s.len(){
      if count + n < len {
        count+=n;
      }else{
        if n % 2 == 0{
        count = count + n - len;
        }else{
        count = count + n - len +1 ;
      }
 }
  
      res.push(chars[count as usize])
      }
    Some(res)

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
