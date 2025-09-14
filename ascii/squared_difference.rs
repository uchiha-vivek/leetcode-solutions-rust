// sum of square differences
use std::io::{self,Read};

fn main(){
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();
  let s = input.trim();
  let mut max_sum:i32 =0;
  let bytes = s.as_bytes();
  for i in 0..bytes.len()-1{
    let diff = bytes[i] as i32 - bytes[i+1] as i32;
    max_sum += diff*diff;
     }
  println!("{}",max_sum);
}