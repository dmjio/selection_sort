use std::vec::Vec;

/// Executes Selection Sort
///
/// # Examples
///
/// ```
/// use selection_sort::selection_sort;
///
/// fn main () {
///   let mut vec = vec![1, 3, 2];
///   selection_sort(&mut vec);
///   println!("{:?}", vec);
///   assert!(vec == [1, 2, 3]);
/// }
/// // [1,2,3]
/// ```
pub fn selection_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    let mut j_min;
    let len = vec.len();
    if len == 0 {
        return;
    }
    for i in 0..len - 1 {
        j_min = i;
        for j in i + 1..len {
            if j >= len {
                break;
            };
            if vec[j] < vec[j_min] {
                j_min = j;
            }
        }
        if j_min != i {
            vec.swap(i, j_min);
        }
    }
}

mod tests {
    #[cfg(test)]
    use super::*;
    use quickcheck::quickcheck;
    quickcheck! {
        fn known_sort_test(vec: Vec<u32>) -> bool {
          let mut a = vec.clone();
          let mut b = vec.clone();
          selection_sort (&mut a);
          b.sort();
          return a == b;
        }
    }
    #[cfg(test)]
    quickcheck! {
      fn idempotency(vec: Vec<u32>) -> bool {
        let mut a = vec.clone();
        let mut b = vec.clone();
        selection_sort (&mut a);
        selection_sort (&mut a);
        selection_sort (&mut b);
        return a == b;
      }
    }
}
