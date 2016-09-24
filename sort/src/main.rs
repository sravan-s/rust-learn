mod insertion_sort;

fn main() {
  let mut arr: [i8; 8] = [7, 1, 8, 2, 3, 5, 9, 2];
  println!("{:?}", arr);
  insertion_sort::welcome();
  insertion_sort::sort(&mut arr);
  println!("{:?}", arr);
}
