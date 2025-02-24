use local_0001::find_mode;

#[test]
fn test_find_mode_empty() {
    let numbers: Vec<i32> = vec![];
    assert_eq!(find_mode(&numbers), (0, 0));
}

#[test]
fn test_find_mode_single_element() {
    let numbers = vec![1];
    assert_eq!(find_mode(&numbers), (1, 1));
}

#[test]
fn test_find_mode_sorted() {
    let numbers = vec![1, 2, 3, 4, 5];
    assert_eq!(find_mode(&numbers), (1, 1));
}

#[test]
fn test_find_mode_unsorted() {
    let numbers = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
    assert_eq!(find_mode(&numbers), (1, 2));
}

#[test]
fn test_find_mode_unsorted_repeated() {
    let numbers = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
    assert_eq!(find_mode(&numbers), (1, 3));
}