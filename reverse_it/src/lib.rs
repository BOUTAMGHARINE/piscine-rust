pub fn reverse_it(k: i32) -> String {
    let mut v = k;
    let mut isnegative = false;
    if v < 0 {
        isnegative = true;
        v*=-1;
    }

    let c = v.to_string();
    let mut d:String = String :: new() ;
    for ch in c.chars().rev(){


     d.push(ch);

}

if isnegative{
    return format!("-{}{}",d,c);
}

let res = format!("{}{}",d,c);

res
    


}