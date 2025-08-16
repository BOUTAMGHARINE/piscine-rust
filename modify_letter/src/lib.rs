pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    let res =   String::new();
    for v in s.char() {
        if v != letter {

            res.push(v)

        }
    }
    res
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {

 let res =   String::new();
    for v in s.char() {
        if v.to_lowercase()!= letter.to_lowercase()  {

            res.push(v)

        }
    }
    res
}

pub fn swap_letter_case(s: &str, letter: char) -> String {


 let res =   String::new();
    for v in s.char() {
        if v.to_lowercase()!= letter.to_lowercase()  {

            res.push(v)

        }
    }
    res


}