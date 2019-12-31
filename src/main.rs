use selection_sort::selection_sort;

pub fn main () {
    let mut vec = vec![10,8,9,6,7,4,5,2,3,1,0];
    println!("{:?}", vec);
    selection_sort (&mut vec);
    println!("length: {}", vec.len());
    println!("{:?}", vec);
}
