use selection_sort::selection_sort;

pub fn main () {
    let mut vec = vec![1, 3, 2, 4];
    println!("{:?}", vec);
    selection_sort (&mut vec);
    println!("length: {}", vec.len());
    println!("{:?}", vec);
}
