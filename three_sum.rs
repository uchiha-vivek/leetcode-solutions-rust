// 3 sum
// f(a,b,c)=a+b+c==0

fn three_sum(nums:Vec<i32>) -> Vec<Vec<i32>> {
    let mut res  = Vec::new();
    let n = nums.len();
    for i in 0..n{
        for j in i+1..n{
            for k in j+1..n{
                let target = nums[i]+nums[j]+nums[k];
                if target == 0 {
                    let mut triplet  = vec![nums[i],nums[j],nums[k]];
                    triplet.sort();
                    if !res.contains(&triplet){
                        res.push(triplet)
                    }
                }
            }
        }
    }
    res
}


// Hashing method
fn three_sum_hashing(nums:Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = HashSet::new();
    let n  = nums.len();
    for i in 0..n{
        let mut sum  = HashSet::new();
        for j in i+1..n{
            let compliment = -nums[i]-nums[j];
            if sum.contains(&compliment){
                let mut triplet = vec![nums[i],nums[j],compliment];
                triplet.sort();
                res.insert(triplet);
            }
            sum.insert(nums[j]);
        }
    }
    res.into_iter().collect()
}

fn three_sum_pointer(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    nums.sort();
    let n = nums.len();

    for i in 0..n {
        if i > 0 && nums[i] == nums[i - 1] {
            continue; // skip duplicates
        }

        let (mut left, mut right) = (i + 1, n - 1);

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum == 0 {
                res.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    res
}

fn main(){
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = three_sum(nums);
    println!("{:?}",result);
}