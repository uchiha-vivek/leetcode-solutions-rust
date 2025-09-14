use std::io::{self,Read};
// checking if a string is smooth
// a string is smooth if the difference between adjacent chars is less than <= n;

// "ca"

fn main(){
 // target difference =4
 let mut input = String::new();
 io::stdin().read_to_string(&mut input).unwrap();
 let s = input.trim();
 let n:i32 = 6;
 // first principle thinking , lets take out max difference first
 let mut max_diff:i32= 0;
 let bytes =  s.as_bytes();
 for i in 0..bytes.len()-1{
   let diff = (bytes[i] as i32 - bytes[i+1] as i32).abs();
   if diff>max_diff{
     max_diff=diff;
   }
 }
 
 println!("{}",max_diff);
 // smooth condition
 if max_diff<=n {
   println!("True");
 }else{
   println!("False");
 }
}