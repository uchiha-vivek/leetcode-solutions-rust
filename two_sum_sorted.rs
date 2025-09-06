// applying two sum when the array is sorted
fn sorted_two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut left = 0;
    let mut right = nums.len()-1;
    while left<right {
        let sum = nums[left]+nums[right];
        if sum==target{
            return vec![left as i32, right as i32];
        } else if sum<target {
            left+=1;
        }else{
            right-=1;
        }
    }
    vec![]
}

fn main(){
    let nums = vec![2,7,11,15];
    let target  = 9;
    let result = sorted_two_sum(nums,target);
    println!("{:?}",result);
}