// https://leetcode.com/problems/search-insert-position/description/

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start=0;
        let mut end= nums.len() as i32-1 ;
        while start<=end{
           let mut mid  = start + (end-start)/2;
            if nums[mid as usize] < target {
                start=mid+1;
            }
            
         else if nums[mid as usize ]>target{
      end=mid-1;
        }
        
     else{
        return mid;
    }
        }
    return start;
}
}