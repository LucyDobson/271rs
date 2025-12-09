//use std::collections::HashMap;
use multimap::MultiMap;
fn main() {
    let mut class = MultiMap::new();

    class.insert("C151", "C280");
    class.insert("C151", "C152");
    class.insert("C151", "D351");


    //assert_eq!(class["C151"], "C280");
    

    println!("{}", class.get("C151").expect("REASON").to_string());
    println!("{}", class.get_vec("C151").expect("REASON"));

}
