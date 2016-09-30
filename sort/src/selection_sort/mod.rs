pub fn sort(array: &mut [i8]) {
  let mut temp;
  for i in 0..array.len() {
    for j in (i + 1)..array.len() {
      if array[j] < array[i] {
        temp = array[i];
        array[i] = array[j];
        array[j] = temp;
      }
    }
  }
}
