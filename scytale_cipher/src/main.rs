use scytale_cipher::*;
fn main() {
    println!("\"sec yCtoadle\" size=2 -> {:?}",
        scytale_decoder("oenset  daa yt hirne et hfea lflosr".to_string(), 2));
    println!("{:?}",scytale_decoder("aebfcgd".to_string(), 2));

    //println!("\"steoca dylCe\" size=4 -> {:?}",
        //scytale_decoder("steoca dylCe".to_string(), 4));
}