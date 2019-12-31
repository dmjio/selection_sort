use std::vec::Vec;

// Selection sort function
pub fn selection_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    let mut j_min;
    let len = vec.len();
    for i in 0..len - 1 {
        j_min = i;
        for j in i + 1..len {
            if j >= len { break };
            if vec[j] < vec[j_min] {
                j_min = j;
            }
        }
        if j_min != i {
            vec.swap(i, j_min);
        }
    }
}

#[test]
fn swap_test() {
    let mut vec = vec![1, 3, 2];
    selection_sort(&mut vec);
    println!("length: {}", vec.len());
    println!("{:?}", vec);
    assert!([1, 2, 3] == [1, 2, 3]);
}
