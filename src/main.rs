mod sort;

use sort::mergesort;
use sort::merge_inversions;
use sort::mergesort_inversions;

mod divandconq;

use divandconq::max_subarray;


fn main() {

    println!("{:?}", mergesort_inversions([2, 3, 8, 6, 1].to_vec().as_slice()));
    let x = vec![1, 2];

    println!("{:?}", max_subarray(vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7].as_slice()));


}
