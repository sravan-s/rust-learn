mod insertion_sort;

fn main() {
  let mut arr: [i8; 4] = [1, 2, 3, 4];
  println!("{:?}", arr);
  insertion_sort::welcome();
  insertion_sort::sort(&mut arr);
}
