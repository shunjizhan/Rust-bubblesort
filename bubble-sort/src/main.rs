fn main() {
    let mut arr = [5, 2, 9, 1, 5, 6];

    print!("before sort:");
    println!("{:?}", arr);

    bubble_sort(&mut arr);

    print!("after sort:");
    println!("{:?}", arr);

    let mut arr2 = vec!["apple", "orange", "banana", "pear"];

    print!("before sort:");
    println!("{:?}", arr2);

    bubble_sort(&mut arr2);

    print!("after sort:");
    println!("{:?}", arr2);
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

#[test]
fn test_bubble_sort_i32() {
    let mut arr1 = [5, 2, 9, 1, 5, 6];
    bubble_sort(&mut arr1);
    assert_eq!(arr1, [1, 2, 5, 5, 6, 9]);

    let mut arr2 = [10, 5, 20, 1];
    bubble_sort(&mut arr2);
    assert_eq!(arr2, [1, 5, 10, 20]);

    let mut arr3 = [3, 3, 3, 3];
    bubble_sort(&mut arr3);
    assert_eq!(arr3, [3, 3, 3, 3]);

    let mut arr4 = [1];
    bubble_sort(&mut arr4);
    assert_eq!(arr4, [1]);

    let mut arr5: [i32; 0] = [];
    bubble_sort(&mut arr5);
    assert_eq!(arr5, []);
}

#[test]
fn test_bubble_sort_string() {
    let mut arr = vec!["apple", "orange", "banana", "pear"];
    bubble_sort(&mut arr);
    assert_eq!(arr, ["apple", "banana", "orange", "pear"]);
}
