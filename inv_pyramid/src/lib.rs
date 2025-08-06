use std::iter::repeat;

pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();
    let value = repeat(v);

    for j in 1..=i {
        let line = value.clone().take(j as usize).collect::<String>();
        let space = repeat(" ").take(j as usize).collect::<String>();
        result.push(format!("{}{}", space, line));
    }

    // clone lines à ajouter dans une nouvelle liste
    let mut mirrored = Vec::new();
    for (j, line) in result.iter().enumerate().rev() {
        if j != result.len() - 1 {
            mirrored.push(line.clone());
        }
    }

    // ajouter à la fin
    result.extend(mirrored);
    result
}
