// reversing string

fn reverse_string(s: &mut String){
    let bytes = unsafe {s.as_bytes_mut()};
    let mut left = 0;
    let mut right = bytes.len().saturating_sub(1);
    while left<right {
        bytes.swap(left,right);
        left+=1;
        right = right.saturating_sub(1);
    }
}

fn main(){
    let mut text = String::from("vivek");
    reverse_string(&mut text);
    println!("{}",text);
}