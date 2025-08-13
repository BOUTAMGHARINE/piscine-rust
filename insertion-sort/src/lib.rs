pub fn insertion_sort(slice: &mut [i32], steps: usize) {
 let mut c:i32 = 0;
    for i in 0..slice.len() {
        while c < i as i32 {
            if slice[i] < slice[i-1]{
               slice[i] = slice[i-1];
               slice[i-1]=slice[i];   //[1,22,8,9,0]
            }
            c+=1;
        }
        c=0;
    }
}