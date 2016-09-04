mod insertion_sort;

fn main() {
  let mut arr: [i8; 4] = [12, 20, 33, 44];
  let len = arr.len();
  println!("{:?}", arr);
  insertion_sort::welcome();
  insertion_sort::sort(&arr);
}
