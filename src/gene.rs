use std::fmt;

mod link;
use link::Link;

#[#[derive(Clone)]]
pub struct Gene {

    id: u32, 
    link: &'static Link<'static>,
    enabled: bool, 

}

impl fmt::Display for Gene { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
            " Gene {} ({}) : {} ", 
            self.id, if self.enabled {"enabled"} else {"disabled"}, 
            self.link)
    }
}


impl Gene {

    pub fn get_id(&self) -> u32 { self.id };

    pub fn get_link(&self) -> &Link { self.link };

    pub fn is_enabled(&self) -> bool { self.enabled };

    pub fn set_enabled(&mut self, value: bool) { self.enabled == value };
    

}

mod node;
use node::Node as Node;


fn main(){

    let n : 'static Node = Node::new(1,String::from(""),Vec::new(),Vec::new()); 
    let ref_n: &'static Node = &n;
    let l : Link = Link {
        src: ref_n,
        dst: ref_n,
        weight: 1.0
    };
    let g : Gene = Gene { 
        id: 3, 
        link: &l,
        enabled: true,
    };

    println!("{}", g);
}