



#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}
#[derive(Debug, PartialEq, Eq)]

pub enum Jacket {
    Black, 
    White,
    Flowers
}
#[derive(Debug, PartialEq, Eq)]

pub enum Hat {
    Snapback,
    Baseball,
    Fedora
}


pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {

    let mut jacket = Jacket::Black;
    let mut hat = Hat::Snapback;

    match formality_level {
        None=>{
            jacket = Jacket::Flowers;
        },
        Some(n)=> {
            if n > 0 {
              jacket= Jacket:: White;
            }else{
                jacket = Jacket::Black;

            }
        }
      

    }
    match invitation_message {
        Ok(_)=> {
            hat = Hat::Fedora;
        },
        _=>{
            if jacket == Jacket::Flowers {
                hat = Hat::Baseball;
            }else {
            hat = Hat::Snapback;
            }
        },
    }

    Outfit {
          jacket,
          hat
    }



}
//My outfit will be: Outfit { jacket: Black, hat: Fedora }