pub fn prev_prime(nbr: u64) -> u64  {

    for i in (2..nbr+1).rev(){

        if is_prime(i){
            return i
        }
    }



return 0
}
pub fn is_prime(nbr :u64)->bool{
    if nbr == 2 {
        return true
    }
    if nbr <= 1{
        return false
    }
    if nbr % 2 == 0 {
        return false
    }
    let limit = (nbr as f64).sqrt() as u64;
    for i in 3..=limit{
        if nbr % i == 0 {
            return false
        }
    }
    true

}
pub fn boucle(nbr :f64){
    let limit = nbr as u64;


    for i in (0..limit+1).rev() {
        println!("{}",i)
    }


}