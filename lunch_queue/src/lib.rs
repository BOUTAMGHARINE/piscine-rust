pub struct Queue {
    pub node: Link,
    pub tab : Vec<Person>
}

pub type Link =Option<Box<Person>>;

pub struct Person {
    pub discount: i32,
    pub name: String,
}

impl Queue {
    pub fn new() -> Queue {
        Self {
            node : None,
            tab : Vec::new()
        }
    }

    pub fn helper(&self){

        let new_node : Link = None;

        for v in self.tab.iter().rev() {
            new_node = Some (Box::new(Person {
                discount: v.discount,
                name : v.name.clone()
            }));
        }
        self.node = new_node ;

    } 
    pub fn add(&mut self, name: String, discount: i32) {
           self.tab.push(Person{discount,name});
           self.helper();


    }
    pub fn invert_queue(&mut self) {
           self.tab.reverse();
           self.helper();
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        self.tab.pop();
        self.helper();

    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
       
    self.tabs.iter().find(|x| x.name == name).map(|x| (x.name.clone() , x.discount))
    }
}