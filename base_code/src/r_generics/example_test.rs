fn get_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn get_largest_universal<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[test]
fn test_vec() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(&number_list);
    println!("the largest number is {}", largest);

    let largest = get_largest_universal(&number_list);
    println!("the largest number is {}", largest);
}
