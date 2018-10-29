//! Search utilities.

/// Get index for binary search, using exact matching.
///
/// Works on sorted input, in O(log(n)) time.
#[allow(dead_code)]
pub(crate) fn binary<T: Ord>(slice: &[T], value: &T)
    -> Option<usize>
{
    let mut first = 0;
    let mut last = slice.len();

    while first < last {
        let pivot = (first+last)/2;
        let x = unsafe { slice.get_unchecked(pivot) };
        if value == x {
            return Some(pivot)
        } else if value < x {
            last = pivot - 1;
        } else {
            first = pivot + 1;
        }

    }

    None
}

/// Get index for linear search, using exact matching.
///
/// Works on unsorted input, in O(n) time.
#[allow(dead_code)]
pub(crate) fn linear<T: Ord>(slice: &[T], value: &T)
    -> Option<usize>
{
    for (i, x) in slice.iter().enumerate() {
        if x == value {
            return Some(i)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_test() {
        let x = [1, 2, 3, 4, 5];
        let index = binary(&x, &3);
        assert_eq!(index, Some(2));

        let index = binary(&x, &5);
        assert_eq!(index, Some(4));

        let index = binary(&x, &6);
        assert_eq!(index, None);
    }

    #[test]
    fn linear_test() {
        let x = [5, 3, 1, 4, 2];
        let index = linear(&x, &3);
        assert_eq!(index, Some(1));

        let index = linear(&x, &5);
        assert_eq!(index, Some(0));

        let index = linear(&x, &6);
        assert_eq!(index, None);
    }
}
