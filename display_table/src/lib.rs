use std::fmt;
// use std::cmp;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
    
    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.headers.len() == 0 {
            return Ok(())
        }

        let mut max : Vec<usize> = self.headers.iter().map(|e| e.chars().count()).collect();
        // write!(f, "{:?}", max)?;
        for array in self.body.iter() {
            for (i, val) in array.iter().enumerate() {
                if max[i] < val.chars().count() {
                    max[i] = val.chars().count();
                }
            }
        }
        // write!(f, "{:?}", max)?;
        for (i, val) in self.headers.iter().enumerate() {
            let tol = max[i];
            let def = tol - val.chars().count();
            if def % 2 == 0 {
                let first = (def/2) +1;
                let last = (def/2)+1;
            write!(f, "|")?;
            write!(f, "{}", " ".repeat(first))?;
            write!(f, "{}", val)?;
            write!(f, "{}", " ".repeat(last))?;
            }else {
                let first = (def/2) +1;
                let last = (def/2)+2;
            write!(f, "|")?;
            write!(f, "{}", " ".repeat(first))?;
            write!(f, "{}", val)?;
            write!(f, "{}", " ".repeat(last))?;
            }
            if i == self.headers.len() -1 {
            write!(f, "|\n")?;
            }
        }
        write!(f, "|")?;
        for (i, val) in max.iter().enumerate() {
            write!(f, "{}", "-".repeat(*val +2))?;
            if i != max.len()-1 {
                write!(f, "+")?;
            }
        }
        write!(f, "|\n")?;

        for array in self.body.iter() {
            for (i, val) in array.iter().enumerate() {
            let tol = max[i];
            let def = tol - val.chars().count();
            if def % 2 == 0 {
                let first = (def/2) +1;
                let last = (def/2)+1;
            write!(f, "|")?;
            write!(f, "{}", " ".repeat(first))?;
            write!(f, "{}", val)?;
            write!(f, "{}", " ".repeat(last))?;
            }else {
                let first = (def/2) +1;
                let last = (def/2)+2;
            write!(f, "|")?;
            write!(f, "{}", " ".repeat(first))?;
            write!(f, "{}", val)?;
            write!(f, "{}", " ".repeat(last))?;
            }
            if i == self.headers.len() -1 {
            write!(f, "|\n")?;
            }
            }
        }

        
       Ok(())
}
}
