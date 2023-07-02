use std::ops::Add;

pub fn max_subarray<'a, T>(array: &'a[T]) -> (&'a[T], T) 
    where T: PartialOrd + Add<Output = T> + Copy {
    if array.len() == 1 {
        return (array, array[0]);
    }

    let half = array.len() / 2;

    let left =  &array[..half];
    let right = &array[half..];

    let (left_ans, left_max) =  max_subarray(left);
    let (right_ans, right_max)= max_subarray(right);

    // WARNING: DUPLICATED CODE 
    // Find left max subarray from middle 
    let mut max_left_cross = array[half-1];
    let mut max_left_index = half-1;
    let mut cur_val =  array[half-1];
    for i in (0..half-1).rev() {
        cur_val = cur_val +  array[i];
        if cur_val > max_left_cross {
            max_left_cross = cur_val;
            max_left_index = i;
        }
    }
    
    // Find right max subarray from middle 
    let mut max_right_cross = array[half];
    let mut max_right_index = half;
    let mut cur_val =  array[half];
    for i in (half+1..array.len()) {
        cur_val = cur_val +  array[i];
        if cur_val > max_right_cross {
            max_right_cross = cur_val;
            max_right_index = i;
        }
    }
    
    let cross_max     = max_left_cross + max_right_cross;

    if left_max <= cross_max && cross_max <= right_max ||
       cross_max <= left_max && left_max <= right_max {
        return (right_ans, right_max);
    } else if cross_max <= right_max && right_max <= left_max || 
              right_max <= cross_max && cross_max <= left_max {
        return (left_ans, left_max);
    } else {
        return (&array[max_left_index..=max_right_index], cross_max)
    }
}