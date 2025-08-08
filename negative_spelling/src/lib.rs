fn spell_number(n: i64) -> String {
    let ones = [
        "", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen"
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety"
    ];

    match n {
        0 => "zero".to_string(),
        1..=19 => ones[n as usize].to_string(),
        20..=99 => {
            let t = n / 10;
            let r = n % 10;
            if r == 0 {
                tens[t as usize].to_string()
            } else {
                format!("{}-{}", tens[t as usize], ones[r as usize])
            }
        }
        100..=999 => {
            let h = n / 100;
            let r = n % 100;
            if r == 0 {
                format!("{} hundred", ones[h as usize])
            } else {
                format!("{} hundred {}", ones[h as usize], spell_number(r))
            }
        }
        1000..=999_999 => {
            let th = n / 1000;
            let r = n % 1000;
            if r == 0 {
                format!("{} thousand", spell_number(th))
            } else {
                format!("{} thousand {}", spell_number(th), spell_number(r))
            }
        }
        1_000_000 => "one million".to_string(),
        _ => "".to_string(),
    }
}
pub fn negative_spell(n: i64) -> String {
    if n > 0 {
        return "error: positive number".to_string();
    }else if n == 0 {
        return "zero".to_string();
    }
    let num = -n; // on prend la valeur positive
    format!("minus {}", spell_number(num))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_numbers() {
        assert_eq!(negative_spell(0), String::from("zero"));
        assert_eq!(negative_spell(-1), String::from("minus one"));
        assert_eq!(negative_spell(-14), String::from("minus fourteen"));
        assert_eq!(negative_spell(-20), String::from("minus twenty"));
        assert_eq!(negative_spell(-22), String::from("minus twenty-two"));
        assert_eq!(negative_spell(-101), String::from("minus one hundred one"));
        assert_eq!(
            negative_spell(-120),
            String::from("minus one hundred twenty")
        );
        assert_eq!(
            negative_spell(-123),
            String::from("minus one hundred twenty-three")
        );
    }
    #[test]
    fn test_medium_numbers() {
        assert_eq!(negative_spell(-1000), String::from("minus one thousand"));
        assert_eq!(
            negative_spell(-1055),
            String::from("minus one thousand fifty-five")
        );
        assert_eq!(
            negative_spell(-1234),
            String::from("minus one thousand two hundred thirty-four")
        );
        assert_eq!(
            negative_spell(-10123),
            String::from("minus ten thousand one hundred twenty-three")
        );
    }
    #[test]
    fn test_long_numbers() {
        assert_eq!(
            negative_spell(-910112),
            String::from("minus nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            negative_spell(-651123),
            String::from("minus six hundred fifty-one thousand one hundred twenty-three")
        );

        assert_eq!(
            negative_spell(-810000),
            String::from("minus eight hundred ten thousand")
        );
        assert_eq!(negative_spell(-1000000), String::from("minus one million"));
    }
    #[test]
    fn test_invalid_numbers() {
        assert_eq!(negative_spell(1), String::from("error: positive number"));
        assert_eq!(
            negative_spell(2390904),
            String::from("error: positive number")
        );
    }
}