fn main() {
    // let mut vec = vec![5, 5, 4, 9, 1, 11, 12, 13, 12, 14, 15];
    // let mut vec2 = vec![5, 5, 4, 9, 1, 11, 12, 13, 12, 14, 15];
    // let vec3 = vec![5, 5, 4, 9, 1, 11, 12, 13, 12, 14, 15];
    // insertion_sort(&mut vec);
    // selection_sort(&mut vec2);
    // let vec3_sorted = merge_sort(vec3);
    // println!("hello {:?} {:?} {:?}", vec, vec2, vec3_sorted);
}

/// Perform an insertion sort to sort the given vector in place.
fn insertion_sort<T>(vec : &mut Vec<T>) where T: Ord + Copy {
    let len = vec.len();

    for mut i in 1..len {
        let key = vec[i];
        let mut index = i - 1;
        while index > 0 && vec[index] > key {
            vec[index + 1] = vec[index];
            index = index - 1;
        }
        if index == 0 {
            vec[1] = vec[0];
            vec[0] = key;
        } else {
            vec[index + 1] = key;
        }
    }
}

/// Perform a selection sort to sort the given vector in place.
fn selection_sort(vec : &mut Vec<i32>) {
    for mut i in 0..vec.len() {
        let mut current_min_index = i;
        for mut j in i+1..vec.len() {
            if vec[j] < vec[current_min_index] {
                current_min_index = j;
            }
        }
        if current_min_index != i {
            let tmp = vec[current_min_index];
            vec[current_min_index] = vec[i];
            vec[i] = tmp;
        }
    }
}

/// Perform a merge sort and returns a sorted vector.
fn merge_sort(vec : &Vec<i32>) -> Vec<i32> {
    let len = vec.len();
    if len > 1 {
        let a_arr : Vec<i32> = vec[0..len/2].to_vec();
        let b_arr : Vec<i32> = vec[len/2..len].to_vec();
        let sorted_a = merge_sort(&a_arr);
        let sorted_b = merge_sort(&b_arr);
        merge_sorted_vectors(&sorted_a, &sorted_b)
    } else {
        vec.clone()
    }
}

/// Merge two sorted vectors into one sorted vector.
fn merge_sorted_vectors(a : &Vec<i32>, b : &Vec<i32>) -> Vec<i32> {
    let a_len = a.len();
    let b_len = b.len();

    let mut res = vec![0; a_len + b_len];

    let mut index_a = 0;
    let mut index_b = 0;
    for mut i in 0..a_len+b_len {
        if a[index_a] > b[index_b] {
            res[i] = b[index_b];
            index_b = index_b + 1;
            if index_b == b_len {
                for _ in index_a..a_len {
                    i = i + 1;
                    res[i] = a[index_a];
                    index_a = index_a + 1;
                }
                return res;
            }
        } else {
            res[i] = a[index_a];
            index_a = index_a + 1;
            if index_a == a_len {
                for _ in index_b..b_len {
                    i = i + 1;
                    res[i] = b[index_b];
                    index_b = index_b + 1;
                }
                return res;
            }
        }
    }
    res
}

fn linear_search(vec : &Vec<i32>, v : i32) -> Option<usize> {
    for mut i in 0..vec.len() {
        if vec[i] == v {
            return Some(i);
        }
    }
    None
}

fn binary_search(vec : &Vec<i32>, v : i32) -> Option<usize> {
    let len = vec.len();
    if len == 0 {
        return None;
    }

    let current_index : usize = (len - 1) / 2;
    if vec[current_index] == v {
        Some(current_index)
    } else if vec[current_index] > v {
        binary_search(&vec[0..current_index].to_vec(), v)
    } else {
        binary_search(&vec[current_index+1..len].to_vec(), v)
            .map(| index | {
                index + current_index + 1
            })

    }
}

/// θ(n*log2(n))-time algorithm that, given a set S of n integers and another
/// integer x, determines whether or not there exist two elements in S whose sum
/// is exactly x.
fn has_two_elements_with_sum(vec : &Vec<i32>, x : i32) -> bool {
    let sorted_vec = merge_sort(vec); // θ(n*log2(n))

    // + θ(n)
    let mut index_1 = 0;
    let mut index_2 = sorted_vec.len() - 1;

    while index_1 < index_2 {
        let sum = sorted_vec[index_1] + sorted_vec[index_2];
        if  sum == x {
            return true;
        } else if sum > x {
            index_2 = index_2 - 1;
        } else {
            index_1 = index_1 + 1;
        }
    }
    false
}

#[test]
fn test_insertion_sort() {
    let mut vec1 = vec![5, 5, 4, 9, 1, 11, 12, 13, 12, 14, 15];
    let mut vec2 = vec![0];
    let mut vec3 : Vec<i32> = vec![];
    insertion_sort(&mut vec1);
    insertion_sort(&mut vec2);
    insertion_sort(&mut vec3);
    assert_eq!(vec1, vec![1, 4, 5, 5, 9, 11, 12, 12, 13, 14, 15]);
    assert_eq!(vec2, vec![0]);
    assert_eq!(vec3, vec![]);
}

#[test]
fn test_selection_sort() {
    let mut vec1 = vec![5, 5, 4, 9, 1, 11, 12, 13, 12, 14, 15];
    let mut vec2 = vec![0];
    let mut vec3 : Vec<i32> = vec![];
    selection_sort(&mut vec1);
    selection_sort(&mut vec2);
    selection_sort(&mut vec3);
    assert_eq!(vec1, vec![1, 4, 5, 5, 9, 11, 12, 12, 13, 14, 15]);
    assert_eq!(vec2, vec![0]);
    assert_eq!(vec3, vec![]);
}

#[test]
fn test_merge_sort() {
    let vec1 = vec![5, 5, 4, 9, 1, 11, 12, 13, 12, 14, 15];
    let vec2 = vec![0];
    let vec3 : Vec<i32> = vec![];
    assert_eq!(merge_sort(&vec1), vec![1, 4, 5, 5, 9, 11, 12, 12, 13, 14, 15]);
    assert_eq!(merge_sort(&vec2), vec![0]);
    assert_eq!(merge_sort(&vec3), vec![]);
}

#[test]
fn test_linear_search() {
    let vec1 = vec![4, 5, 6, 8, 9];
    let vec2 = vec![1, 1, 2];
    let vec3 : Vec<i32> = vec![];

    assert_eq!(linear_search(&vec1, 0), None);
    assert_eq!(linear_search(&vec1, 1), None);
    assert_eq!(linear_search(&vec1, 2), None);
    assert_eq!(linear_search(&vec1, 3), None);
    assert_eq!(linear_search(&vec1, 4), Some(0));
    assert_eq!(linear_search(&vec1, 5), Some(1));
    assert_eq!(linear_search(&vec1, 6), Some(2));
    assert_eq!(linear_search(&vec1, 7), None);
    assert_eq!(linear_search(&vec1, 8), Some(3));
    assert_eq!(linear_search(&vec1, 9), Some(4));
    assert_eq!(linear_search(&vec1, 10), None);

    assert_eq!(linear_search(&vec2, 0), None);
    assert_eq!(linear_search(&vec2, 1), Some(0));
    assert_eq!(linear_search(&vec2, 2), Some(2));
    assert_eq!(linear_search(&vec2, 3), None);

    assert_eq!(linear_search(&vec3, 0), None);
    assert_eq!(linear_search(&vec3, 1), None);
    assert_eq!(linear_search(&vec3, 2), None);
}

#[test]
fn test_binary_search() {
    let vec1 = vec![4, 5, 6, 8, 9];
    let vec2 = vec![1, 1, 2];
    let vec3 : Vec<i32> = vec![];

    assert_eq!(binary_search(&vec1, 0), None);
    assert_eq!(binary_search(&vec1, 1), None);
    assert_eq!(binary_search(&vec1, 2), None);
    assert_eq!(binary_search(&vec1, 3), None);
    assert_eq!(binary_search(&vec1, 4), Some(0));
    assert_eq!(binary_search(&vec1, 5), Some(1));
    assert_eq!(binary_search(&vec1, 6), Some(2));
    assert_eq!(binary_search(&vec1, 7), None);
    assert_eq!(binary_search(&vec1, 8), Some(3));
    assert_eq!(binary_search(&vec1, 9), Some(4));
    assert_eq!(binary_search(&vec1, 10), None);

    assert_eq!(binary_search(&vec2, 0), None);
    assert_eq!(binary_search(&vec2, 2), Some(2));
    assert_eq!(binary_search(&vec2, 3), None);

    assert_eq!(binary_search(&vec3, 0), None);
    assert_eq!(binary_search(&vec3, 1), None);
    assert_eq!(binary_search(&vec3, 2), None);
}

#[test]
fn test_has_two_elements_with_sum() {
    let vec1 = vec![4, 5, 6, 8, 9];
    assert_eq!(has_two_elements_with_sum(&vec1, 9), true);
    assert_eq!(has_two_elements_with_sum(&vec1, 4), false);
    assert_eq!(has_two_elements_with_sum(&vec1, 11), true);
    assert_eq!(has_two_elements_with_sum(&vec1, 12), true);
    assert_eq!(has_two_elements_with_sum(&vec1, 13), true);
    assert_eq!(has_two_elements_with_sum(&vec1, 14), true);
    assert_eq!(has_two_elements_with_sum(&vec1, 15), true);
    assert_eq!(has_two_elements_with_sum(&vec1, 16), false);
}
