struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");
    let x: i32 = 5;
    let y: f64 = 3.14;
    let name: &str = "Rust";

    println!("x = {}, y = {}, name = {}", x, y, name);
    println!("x + y = {}", add(x, y as i32));

    let x = 5;
    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is zero");
    }
    loop {
        println!("again!");
        break;
    }
    let mut counter = 0;
    while counter < 10 {
        println!("counter = {}", counter);
        counter += 1;
    }
    let a = 5;
    match a {
        0 => println!("a is zero"),
        1..=5 => println!("a is between 1 and 5"),
        _ => println!("a is something else"),
    }

    let p = Point { x: 1, y: 2 }; // p.x = 1, p.y = 2
    println!("p.x = {}", p.x); // 1
    println!("p.y = {}", p.y); // 2
    let p2 = Point { x: 3, ..p }; // p2.y = p.y
    println!("p2.y = {}", p2.y); // 2

    let p3 = Point { x: 3, ..p2 }; // p3.y = p2.y
    println!("p3.y = {}", p3.y); // 2

    let Human { name } = Human { name: "John" };
    println!("name = {}", name);
    Human { name: "John" }.talk();

    let Dog { name } = Dog { name: "Rex" };
    println!("name = {}", name);
    Dog { name: "Rex" }.talk();

    let Cat { name } = Cat { name: "Misty" };
    println!("name = {}", name);
    Cat { name: "Misty" }.talk();
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn new(name: &'static str) -> Human {
        Human { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn new(name: &'static str) -> Cat {
        Cat { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

struct Dog {
    name: &'static str,
}

impl Animal for Dog {
    fn new(name: &'static str) -> Dog {
        Dog { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says woof", self.name);
    }
}
