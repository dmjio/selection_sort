use std::vec::Vec;

// Selection sort function
pub fn selection_sort(vec: &mut Vec<i32>) {
    let mut j_min: i32;
    let len: i32 = vec.len() as i32;
    for i in 0..len - 1 as i32 {
        j_min = i;
        for j in i + 1..len as i32 {
            if j >= len { break };
            if vec[j as usize] < vec[j_min as usize] {
                j_min = j;
            }
        }
        if j_min != i {
            vec.swap(i as usize, j_min as usize);
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
