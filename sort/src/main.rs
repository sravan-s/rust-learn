mod insertion_sort;

fn main() {
  let mut arr: [i8; 4] = [2, 1, 8, 4];
  println!("{:?}", arr);
  insertion_sort::welcome();
  insertion_sort::sort(&mut arr);
}
