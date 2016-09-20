use std::thread;
use std::time;

struct Philosopher {
  name: String
}

// Creates an assoiated function new within Philosopher
impl Philosopher {
  // @params#new => name &str
  // Returns a Philosopher
  fn new(name: &str) -> Philosopher {
    Philosopher {
      name: name.to_string(),
    }
  }

  fn eat(&self) {
    println!("{} has started eating", self.name);
    thread::sleep(time::Duration::from_millis(1000));
    println!("{} has finished eating", self.name);
  }
}

fn main() {
  let philosophers = vec![
    Philosopher::new("Baruch Spinoza"),
    Philosopher::new("Gilles Deleuze"),
    Philosopher::new("Karl Marx"),
    Philosopher::new("Friedrich Nietzsche"),
    Philosopher::new("Michel Foucault")
  ];

  let handles: Vec<_> = philosophers.into_iter().map(|p| {
    thread::spawn(move || {
      p.eat();
    })
  }).collect();

  for h in handles {
    h.join().unwrap();
  }
}
