use std::cmp::PartialEq;
/// Takes a vector and a size integer and returns a vector of vectors
/// whose length === size
///
/// # Example
/// ```
/// use r_dash::vec_utils::chunk;
///
/// let v = vec![1, 2, 3, 4];
/// let size = 2;
/// let result = chunk(v, size);
/// assert_eq!(result, vec![vec![1, 2], vec![3, 4]])
/// ```
pub fn chunk<T>(original_vector: Vec<T>, size: u32) -> Vec<Vec<T>> {
  let mut final_vec = Vec::new();
  let mut editing_vec = Vec::new();
  for entry in original_vector {
    if editing_vec.len() == size as usize {
      final_vec.push(editing_vec);
      editing_vec = Vec::new();
      editing_vec.push(entry);
    } else {
      editing_vec.push(entry);
    }
  }
  if editing_vec.len() > 0 {
    final_vec.push(editing_vec)
  }
  final_vec
}

/// Creates an array of values that exist the first vector, but do not
/// exist in the second.
///
/// #Example
/// ```
/// use r_dash::vec_utils::difference;
///
/// let v1 = vec![1, 2, 3];
/// let v2 = vec![2, 3, 4];
/// let result = difference(v1, v2);
/// assert_eq!(result, vec![1]);
/// ```
pub fn difference(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
  // TODO: This needs to work with generics. Maybe need to do a test of the value
  // to make sure it can be compared?
  let mut return_vec = Vec::new();
  for item in v1 {
    if let None = v2.iter().position(|&i| i == item) {
      return_vec.push(item);
    }
  }
  return_vec
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_chunk_num() {
    let v = vec![1, 2, 3, 4, 5];
    let test_vec = chunk(v, 2);
    let final_vec = vec![vec![1, 2], vec![3, 4], vec![5]];
    assert_eq!(test_vec, final_vec);
  }

  #[test]
  fn test_chunk_str() {
    let v = vec!["Hello", "My", "Name", "Is", "Potato"];
    let test_vec = chunk(v, 3);
    let final_vec = vec![vec!["Hello", "My", "Name"], vec!["Is", "Potato"]];
    assert_eq!(test_vec, final_vec);
  }

  #[test]
  fn test_chunk_vec() {
    let v = vec![vec![1,2], vec![1,2], vec![1,2], vec![1,2]];
    let test_vec = chunk(v, 3);
    let final_vec = vec![vec![vec![1, 2], vec![1, 2], vec![1, 2]], vec![vec![1, 2]]];
    assert_eq!(test_vec, final_vec);
  }

  #[test]
  fn test_difference_num() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![2, 3];
    let test_vec = difference(v1, v2);
    let final_vec = vec![1];
    assert_eq!(test_vec, final_vec);
  }
}
