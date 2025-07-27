pub fn next_prime(nbr: u64) -> u64 {
    let mut n = nbr;
    if is_prime(n){
        println!("hhhhhhh");
        return n
    }

    while is_prime(n) == false {

        n+=1;
 }
 n

}

pub fn is_prime(nbr: u64) -> bool {

    if nbr <= 1 {
        return false
    }
    if nbr == 2{
        return true
    }
    if nbr % 2 == 0 {
        return false
    }
    let limit = (nbr as f64).sqrt() as u64;
    for i in 3..=limit{
        if nbr%i==0{
            
            return false
        }
    }
    return true

}



