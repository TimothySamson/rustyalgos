use std::fmt::Debug;


fn merge<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T>{
    let mut res: Vec<T> = vec![];
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            res.push(a[i].clone());
            i += 1;
        } else {
            res.push(b[j].clone());
            j += 1;
        } 
    }

    res.extend_from_slice(&a[i..]);
    res.extend_from_slice(&b[j..]);


    return res;

}

pub fn mergesort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    if list.len() <= 1 {
        return list.to_vec()
    }

    let half = list.len() / 2;

    return merge(
            &mergesort(&list[..half]), 
            &mergesort(&list[half..])
    );
}

// Expects a sorted array!!
pub fn binary_search<T: Ord + Clone>(x: &T, list: &[T]) -> bool {
    if list.len() == 1 { return list[0] == *x}

    if list[0] <= *x && *x <= list.last().unwrap().clone() {
        let half = list.len() / 2; 
        return  binary_search(x, &list[..half]) || binary_search(x, &list[half..])
    } else {
        return false
    }
}

// creates a sorted list and counts the inversions between the two lists
pub fn merge_inversions<T: Ord + Clone>(a: &[T], b: &[T]) -> (Vec<T>, u64){
    let mut res: Vec<T> = vec![];
    let (mut i, mut j) = (0, 0);
    let mut inversions = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            res.push(a[i].clone());

            i += 1;
        } else {
            res.push(b[j].clone());
            inversions += a.len() - i; // the only difference
            j += 1;
        } 
    }

    res.extend_from_slice(&a[i..]);
    res.extend_from_slice(&b[j..]);

    return (res, inversions as u64);
}

// CAN I USE A MONAD??
// returns a sorted list and the number of inversions
pub fn mergesort_inversions<T: Ord + Clone>(list: &[T]) -> (Vec<T>, u64) {
    if list.len() <= 1 {
        return (list.to_vec(), 0);
    }

    let half = list.len() / 2;

    let (a, inv_a) = mergesort_inversions(&list[..half]);
    let (b, inv_b) = mergesort_inversions(&list[half..]);
    let (x, inv_x) = merge_inversions(a.as_slice(), b.as_slice());

    return (x, inv_a + inv_b + inv_x);


}
