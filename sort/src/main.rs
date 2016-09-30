mod insertion_sort;
mod selection_sort;

fn main() {
  let mut arr: [i8; 8] = [7, 1, 8, 2, 3, 5, 9, 2];
  println!("{:?}", arr);
  selection_sort::sort(&mut arr);
  println!("{:?}", arr);
}
