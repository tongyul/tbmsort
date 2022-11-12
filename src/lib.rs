use rand::prelude::*;

fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    (1..arr.len()).all(|i| arr[i-1] <= arr[i])
}

fn tbmsort<T: PartialOrd + Send>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    while !is_sorted(arr) {
        std::thread::scope(|s| {
            for half in arr.chunks_mut(arr.len() / 2) {
                s.spawn(|| tbmsort(half));
            }
        });
        tbmerge(arr);
    }
}

fn tbmerge<T: PartialOrd>(arr: &mut [T]) {
    arr.shuffle(&mut rand::thread_rng());
}

pub trait TBMSort<T>
where
    Self: AsMut<[T]>,
    T: PartialOrd + Send,
{
    fn tbmsort(&mut self) {
        tbmsort(self.as_mut());
    }
}

impl<T, U> TBMSort<T> for U
where
    U: AsMut<[T]> + ?Sized,
    T: PartialOrd + Send,
{}

#[cfg(test)]
mod test {
    use super::*;

    fn sort_test_array<T, const N: usize>(mut arr: [T; N])
    where
        T: Ord + Send + Clone + std::fmt::Debug,
    {
        let mut arr2 = arr.clone();
        arr.tbmsort();
        arr2.sort();
        assert_eq!(arr, arr2);
    }

    #[test]
    fn few_elements() {
        sort_test_array::<usize, 0>([]);
        sort_test_array::<usize, 1>([42,]);
    }

    #[test]
    fn all_permutations_3() {
        sort_test_array([1, 2, 3]);
        sort_test_array([1, 3, 2]);
        sort_test_array([2, 1, 3]);
        sort_test_array([2, 3, 1]);
        sort_test_array([3, 1, 2]);
        sort_test_array([3, 2, 1]);
    }

    #[test]
    fn all_permutations_4() {
        sort_test_array([1, 2, 3, 4]);
        sort_test_array([1, 2, 4, 3]);
        sort_test_array([1, 3, 2, 4]);
        sort_test_array([1, 3, 4, 2]);
        sort_test_array([1, 4, 2, 3]);
        sort_test_array([1, 4, 3, 2]);
        sort_test_array([2, 1, 3, 4]);
        sort_test_array([2, 1, 4, 3]);
        sort_test_array([2, 3, 1, 4]);
        sort_test_array([2, 3, 4, 1]);
        sort_test_array([2, 4, 1, 3]);
        sort_test_array([2, 4, 3, 1]);
        sort_test_array([3, 1, 2, 4]);
        sort_test_array([3, 1, 4, 2]);
        sort_test_array([3, 2, 1, 4]);
        sort_test_array([3, 2, 4, 1]);
        sort_test_array([3, 4, 1, 2]);
        sort_test_array([3, 4, 2, 1]);
        sort_test_array([4, 1, 2, 3]);
        sort_test_array([4, 1, 3, 2]);
        sort_test_array([4, 2, 1, 3]);
        sort_test_array([4, 2, 3, 1]);
        sort_test_array([4, 3, 1, 2]);
        sort_test_array([4, 3, 2, 1]);
    }

    #[test]
    fn how_about_8_elements() {
        sort_test_array([1, 0, 2, 3, 4, 5, 6, 7]);
    }
}
