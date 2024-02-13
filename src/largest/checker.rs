pub fn largest_num(nums : &[i32]) -> &i32 {
    let mut largest = &nums[0] ; 

    for num in nums {
        if num > largest {
            largest = num ; 
        }
    }
    print!("The largest number is {}" , largest);
    largest
}