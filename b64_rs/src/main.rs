// read in the file given by file name
// characterize each letter in binary
// break the binary into 6's and add 0s if incomplete
// covert this to b64 letters
// letter -> decimal -> binary -> chunks of 6 bits -> b64
use std::env; 
use std::fs;
const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn to_base64(input: &[u8]) -> String
{
    let mut answer : String = String::new();
    let mut pos : usize = 0;
    let mut output_count : usize = 0;

    while pos < input.len()
    {
        let b1 : u8 = input[pos];
        
        let mut b2 : u8 = 0;
        if pos + 1 < input.len()
        {
            b2 = input[pos + 1];
        }
        
        let mut b3 : u8 = 0;
        if pos + 2 < input.len()
        {
            b3 = input[pos + 2];
        }
        
        // This should now the three bytes inside a 32 bit space
        let combo : u32 = ((b1 as u32) << 16)| ((b2 as u32) << 8)| ((b3 as u32));

        let fourth : usize = (combo & 0b111111) as usize;
        let third : usize = ((combo >> 6) & 0b111111) as usize;
        let second : usize = ((combo >> 12) & 0b111111) as usize;
        let first : usize = ((combo >> 18) & 0b111111) as usize;
         
        answer.push(BASE64_CHARS[first] as char);
        
        answer.push(BASE64_CHARS[second] as char);

        if (pos + 1) < input.len()
        {
            answer.push(BASE64_CHARS[third] as char);
        }
        if (pos + 1) >= input.len()
        {
            answer.push('=');   
        }


        if (pos + 2) < input.len()
        {
            answer.push(BASE64_CHARS[fourth] as char);
        }
        if (pos + 2) >= input.len()
        {
            answer.push('=');
        }
         
        pos += 3;
        output_count += 4;

        if output_count >= 76
        {
          //  answer.push('\n');
            output_count = 0;
        }
    }   
    answer


}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    

    // The first argument is the path that was used to call the program.
   // println!("My path is {}.", args[0]);


   // println!("In file {file_path}");
    
    let contents = fs::read(file_path)
        .expect("Should have been able to read the file");
   // println!("With text:\n{String::from_utf8(contents)}");

    let output_string : String = to_base64(&contents);

    println!("{}", output_string);
}

