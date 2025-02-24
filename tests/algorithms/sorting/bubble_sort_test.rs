use super::super::super::super::algorithms::sorting::bubble_sort::bubble_sort;

#[cfg(test)]
mod tests {

    #[test]
    fn test_bubble_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_bubble_sort_single_element() {
        let mut arr = vec![1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn test_bubble_sort_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_random() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_bubble_sort_duplicates() {
        let mut arr = vec![2, 3, 2, 3, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![2, 2, 2, 3, 3]);
    }

    #[test]
    fn test_bubble_sort_negative() {
        let mut arr = vec![3, -1, 4, -1, 5, -9, 2, -6, 5, -3, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![-9, -6, -3, -1, -1, 2, 3, 4, 5, 5, 5]);
    }
}