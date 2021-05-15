use std::cmp;

fn merge<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
    where T : cmp::PartialOrd + Copy
{
    let mut result = vec![];

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

fn _sort<T>(a: &Vec<T>, start: usize, end: usize) -> Vec<T>
    where T : cmp::PartialOrd + Copy
{
    if start == end {
        return vec![a[start]];
    }

    if (end - start) == 1 {
        let first = a[start];
        let second = a[end];

        if first > second {
            return vec![second, first];
        } else {
            return vec![first, second];
        }
    }

    let middle = (end + start) / 2;

    let first_half = _sort(a, start, middle);
    let second_half = _sort(a, middle + 1, end);

    return merge(&first_half, &second_half);
}

pub fn merge_sort<T>(a: &Vec<T>) -> Vec<T>
    where T : cmp::PartialOrd + Copy
{
    let sorted = _sort(a, 0, a.len() - 1);

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_size_vec() {
        assert_eq!(
            merge_sort(&vec![1]),
            vec![1],
        );
    }

    #[test]
    fn two_size_vec() {
        assert_eq!(
            merge_sort(&vec![2, 1]),
            vec![1, 2],
        );
    }

    #[test]
    fn base_sort() {
        assert_eq!(
            merge_sort(&vec![1, 2, 3, 5, 4]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn inverted_array() {
        assert_eq!(
            merge_sort(&vec![5, 4, 3, 2, 1]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn large_vec() {
        let mut vec = (0..10).collect::<Vec<_>>();

        vec.reverse();

        assert_eq!(
            merge_sort(&vec),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
