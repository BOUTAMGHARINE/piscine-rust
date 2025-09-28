
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    for arg in &args[1..] {
        println!("{}", if is_bracketed_ok(arg) { "OK" } else { "Error" });
    }
}


fn is_bracketed_ok(s: &str) -> bool {

    let mut stack = Vec::new();

    for ch in s.chars() {


        match ch {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | ']' | '}' => {
                if Some(ch) != stack.pop(){
                    return false
                }
               },
            _  => {}
        }
    }

    stack.is_empty()
   
}