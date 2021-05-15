use std::cmp::PartialOrd;

fn merge<T>(a: &[T], b: &[T]) -> Vec<T>
    where T : PartialOrd + Copy
{
    let mut result = Vec::with_capacity(a.len() + b.len());

    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    while i < a.len() {
        result.push(a[i]);
        i += 1;
    }

    while j < b.len() {
        result.push(b[j]);
        j += 1;
    }

    result
}

fn _sort<T>(a: &[T], start: usize, end: usize) -> Vec<T>
    where T : PartialOrd + Copy
{
    if start == end {
        return vec![a[start]];
    }

    let middle = (end + start) / 2;

    let first_half = _sort(a, start, middle);
    let second_half = _sort(a, middle + 1, end);

    merge(&first_half, &second_half)
}

pub fn merge_sort<T>(a: &[T]) -> Vec<T>
    where T : PartialOrd + Copy
{
    if a.is_empty() {
        return vec![];
    }

    _sort(a, 0, a.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        assert_eq!(
            merge_sort::<i32>(&[]),
            [],
        );
    }

    #[test]
    fn one_size_vec() {
        assert_eq!(
            merge_sort(&[1]),
            [1],
        );
    }

    #[test]
    fn two_size_vec() {
        assert_eq!(
            merge_sort(&[2, 1]),
            [1, 2],
        );
    }

    #[test]
    fn simple_sort() {
        assert_eq!(
            merge_sort(&[1, 2, 3, 5, 4]),
            [1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn inverted_array() {
        assert_eq!(
            merge_sort(&[5, 4, 3, 2, 1]),
            [1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn large_vec() {
        let mut vec = (0..20).collect::<Vec<_>>();

        vec.reverse();

        assert_eq!(
            merge_sort(&vec),
            (0..20).collect::<Vec<_>>()
        );
    }
}
