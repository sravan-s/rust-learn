pub fn sort(array_to_sort: &mut [i8]) {
  let mut temp: i8;
  println!("length: {}", array_to_sort.len());
  for j in 2..array_to_sort.len() {
    println!("j: {}", j);
    for k in (1..(j - 1)).rev(){
      println!("j: {} and k: {}", array_to_sort[j], array_to_sort[k]);
      println!("k: {}", k);
      if array_to_sort[j] < array_to_sort[k] {
        temp = array_to_sort[j];
        array_to_sort[j] = array_to_sort[k];
        array_to_sort[k] = temp;
      }
    }
  }
  println!("{:?}", array_to_sort);
}

pub fn welcome() {
  println!("Welcome to insertion sort");
}
