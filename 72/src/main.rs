use std::env;
use bignum::ix;
/*
 * pub fn u64_to_ix(val: u64) -> ix {
      return ix {
          sign: false,
          vals: vec![val],
      };
  }
  
  fn add_ix(n1: &ix,n2: &ix) -> ix{
      //n1 and n2 convert them both to u128
      let answer : ix = ix::new();
      /*let mut count : u8 = 2;
      let mut remainder : u8 = 0;
  
      let p1 :u64 = (n1 & 0b11);
      let p2 :u64 = (n1 >> count) & 0b11; 
  
      let s1 :u64 = (n2 & 0b11);
      let s2 :u64 = (n2 >> count) & 0b11; 
      */
  
  
      answer
  }

 *
 *
 */



/*
fn add_ix(n1: &ix,n2: &ix) -> ix{
    let answer : ix = ix::new();
    answer
}
*/

fn split(hex1: &String) -> Vec<u64> {
    
    let mut values1: Vec<u64> = Vec::new();
    //let args: Vec<String> = env::args().collect();
    //let hex1: &String  = &args[1]; 

    println!("{}", hex1);

    // takes a hex and finds the smallestmultiple of 16 it can fit into
    //turn this all into a function
    let pad_len1 : usize = (((hex1.trim_start_matches("0x").len() - 1)/16)+1) * 16;
    

    println!("Pad length: {}", pad_len1);

    // takes the string and fits it into multiple of 16, padding with 0s
    let pad_hex1 :  String = format!("{:0>width$}", hex1.trim_start_matches("0x"), width = pad_len1);

    println!("{}", pad_hex1);
    println!("Pad hex: {}", pad_hex1.len());
    let mut i: usize = 0;
    while i < pad_len1 {
    // splits every 16 characters into it's own u64 vec element
        let slice : &str = &pad_hex1[i..i+16];
        values1.push(u64::from_str_radix(slice,16).unwrap());
        i+=16;
        println!("{}", slice);
    
        return value1;
    }


fn main() {
    
    let args: Vec<String> = env::args().collect();
    let hex1: &String  = &args[1]; 
    split(hex1);

    /*
    let mut values1: Vec<u64> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let hex1: &String  = &args[1]; 

    println!("{}", hex1);

    // takes a hex and finds the smallestmultiple of 16 it can fit into
    //turn this all into a function
    let pad_len1 : usize = (((hex1.trim_start_matches("0x").len() - 1)/16)+1) * 16;
    

    println!("Pad length: {}", pad_len1);

    // takes the string and fits it into multiple of 16, padding with 0s
    let pad_hex1 :  String = format!("{:0>width$}", hex1.trim_start_matches("0x"), width = pad_len1);

    println!("{}", pad_hex1);
    println!("Pad hex: {}", pad_hex1.len());
    let mut i: usize = 0;
    while i < pad_len1 {
    // splits every 16 characters into it's own u64 vec element
        let slice : &str = &pad_hex1[i..i+16];
        values1.push(u64::from_str_radix(slice,16).unwrap());
        i+=16;
        println!("{}", slice);*/
    }






}
