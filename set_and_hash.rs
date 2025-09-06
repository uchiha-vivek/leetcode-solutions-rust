// understanding hasing in rust
use std::collections::{HashMap,HashSet};

fn main(){
    let mut map = HashMap::new();
    map.insert("chain link 1",3);
    map.insert("chain link 2",4);
    println!("{:?}",map.get("chain link 1"));
    
    let mut set = HashSet::new();
    set.insert("chain link 1");
    set.insert("chain link 2");
    println!("{}",set.contains("chain link 1"));
}