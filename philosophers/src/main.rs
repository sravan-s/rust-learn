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

  for p in philosophers {
    p.eat();
  }
}
