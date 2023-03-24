fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for &num in numbers {
        match sum.checked_add(num) {
            Some(new_sum) => sum = new_sum,
            None => return None,
        }
    }

    Some(sum)
}

fn main() {
    let arr = &[0, 23, 3, 15, 123];
    print!("calculating sum of {:?} => ", arr);

    match sum_u32(arr) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred"),
    }

    let arr = &[0, 23, 3, 15, 123, u32::MAX];
    print!("calculating sum of {:?} => ", arr);

    match sum_u32(arr) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred"),
    }
}
