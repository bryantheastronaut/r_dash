pub fn chunk<T>(orignal_vector: Vec<T>, size: i32) -> Vec<Vec<T>> {
  let mut v = Vec::new();
  let v1 = vec![1,2,3];
  let v2 = vec![4,5,6];
  for i in v1 {
    println!("{:?}", i);
  }
  v
}

#[test]
fn test_chunk() {
  assert_eq!(chunk(vec![1,2,4], 22), vec![vec![1,2,3], vec![4,5,6]]);
}