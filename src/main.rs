use std::{
    sync::{Arc, Mutex},
    thread,
};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}
impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left,
            right,
        }
    }
    // fn loop_philosophers(&Self){
    //     Self.next()
    // }
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is done eating", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating", self.name);
    }
}
struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });
    let p1 = Philosopher::new("Pedro", 0, 1);
    let p2 = Philosopher::new("Henrique", 1, 2);
    let p3 = Philosopher::new("Tiago", 2, 3);
    let p4 = Philosopher::new("Max", 3, 4);
    let p5 = Philosopher::new("Miguel", 0, 4);
    let philosopher = vec![p1, p2, p3, p4, p5];
    let handles: Vec<_> = philosopher
        .into_iter()
        .map(|p| {
            let table = table.clone();
            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
