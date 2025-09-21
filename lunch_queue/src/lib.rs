#[derive(Debug)]
pub struct Queue {
    pub node: Link,
    pub tab : Vec<Person>
}

pub type Link =Option<Box<Person>>;
#[derive(Debug)]

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
    pub fn helper(& mut self) {
        let mut node : Link = None;
        for v in self.tab.iter().rev(){
          node = Some(Box::new(Person {
            discount :v.discount,
            name : v.name.clone()
          }))


        } 
        self.node = node;
    }
    pub fn add(&mut self, name: String, discount: i32) {
        self.tab.insert(0,Person{discount,name});
        self.helper();

    }
    pub fn invert_queue(&mut self) {
        self.tab.reverse();
        self.helper();

    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let persone = self.tab.pop();
        self.helper();
        persone.map(|x| (x.name.clone(),x.discount))

    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        self.tab.iter().find(|x| x.name ==name).map(|x| (x.name.clone(),x.discount))

    }
}