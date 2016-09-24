pub fn welcome() {
  println!("Welcome to insertion sort");
}

// Inspired from
// http://stackoverflow.com/questions/30965257/why-is-my-rust-implementation-of-insertionsort-slower-than-my-c-version

pub fn sort(array: &mut [i8]) {
  let mut temp;
  for i in 1..array.len() {
    temp = array[i];
    for j in (0..(i)).rev() {
      if temp < array[j] {
        array[j + 1] = array[j];
      } else {
        array[j + 1] = temp;
        break;
      }
      if j == 0 {
        array[0] = temp;
        break;
      }
    }
  }
}
