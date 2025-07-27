pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let mut  max : i32=i32::MIN;
    let mut  min :i32=i32::MAX;
    if nb_1 - nb_2 > 0 {
        max = nb_1;
        min =nb_2;
    }else {
        max = nb_2;
        min =nb_1;
    }
    if max < nb_3 {
        max = nb_3;
    } else if min > nb_3{
        min = nb_3;

    }
    (min,max)
}