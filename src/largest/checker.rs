use std::fmt::Display ; 

pub fn largest_num<T>(nums : &[T]) -> &T  
where T : PartialOrd + Display {
    let mut largest = &nums[0] ; 

    for num in nums {
        if num > largest {
            largest = num ; 
        }
    }
    print!("The largest number is {}" , largest);
    largest
}