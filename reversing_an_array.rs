// reversing an array

fn reverse(nums: &mut Vec<i32>){
    let mut left = 0;
    let mut right = nums.len()-1;
    while left<right {
        nums.swap(left,right);
        left+=1;
        right-=1;
    }
}


fn main(){
    let mut arr = vec![1,2,3,4,5];
    reverse(&mut arr);
    println!("{:?}",arr);
}