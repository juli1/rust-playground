trait Animal {
    fn make_noise(&self);
}

struct Cat {
    name : String,
}

struct Dog {
    name : String,
}

impl Cat {
    pub fn new(n : &str) -> Cat {
        Cat { name : String::from(n)}
    }
}

impl Dog {
    pub fn new(n : &str) -> Dog {
        Dog { name : String::from(n)}
    }
}


impl Animal for Dog {
    fn make_noise (&self) {
        println!("{} barks", self.name);
    }
}


impl Animal for Cat {
    fn make_noise (&self) {
        println!("{} miawww", self.name);
    }
}


fn main() {
    println!("Starting main");
    let chewie = Dog::new("Chewie");
    let boubou = Cat::new("Boubou");
    chewie.make_noise();
    boubou.make_noise();
}
