#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Self {
            headers : Vec::new(),
            body : Vec::new()
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }

    pub fn filter_col<T>(&self, filter: T) -> Option<Self> where T : Fn(&str)->bool, {
        let mut tabl = Table::new();
        let mut indice = Vec::new();
        for (i,v) in self.headers.iter().enumerate() {
            if filter(v) {
                tabl.headers.push(v.to_string());
                indice.push(i);
            }
        }
        if tabl.headers.is_empty() {
            return None
        }
        for (_,row) in self.body.iter().enumerate() {
            let mut new = Vec::new();
            for (j,value) in row.iter().enumerate() {
                if indice.contains(&j) {
                    new.push(value.to_string());
                }
                
            }
            tabl.body.push(new.clone());
        }
        Some (tabl)
}

pub fn filter_row<F>(&self, col_name: &str, filter: F) -> Option<Self>
where
    F: Fn(&str) -> bool,
{
    // Trouver l’indice de la colonne
    let mut index : Option<usize> = None;
    for (i,v) in self.headers.iter().enumerate() {
        if v == col_name {
            index = Some(i);
            break;
        }
    }

 

    let mut tabl = Table::new();

    // On garde toutes les colonnes, donc on clone les headers
    tabl.headers = self.headers.clone();

    // Filtrer les lignes
    for row in self.body.iter() {
        if let Some(value) = row.get(index.unwrap()) {
            if filter(value) {
                tabl.body.push(row.clone());
            }
        }
    }

    // Si aucune ligne retenue, on retourne None
    if tabl.body.is_empty() {
        return None;
    }

    Some(tabl)
}

}