pub fn largest_num() {
    let nums = vec![10 , 30 , 20 , 1 , 3 , 0] ; 
    let mut largest = &nums[0] ; 

    for num in &nums {
        if num > largest {
            largest = num ; 
        }
    }
    print!("The largest number is {}" , largest);
}