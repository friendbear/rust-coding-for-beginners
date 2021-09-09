trait Noise {
    fn make_noize(&self);
}
trait Racer {
    //    fn go(&self);
    fn is_ready(&self) -> bool;
    //    fn checkpoint(&self, position: i32);
}
struct Doc {}

struct Human {}

impl Noise for Human {
    fn make_noize(&self) {
        println!("⚡")
    }
}

impl Noise for Doc {
    fn make_noize(&self) {
        println!("わん!")
    }
}

fn hello(noisy: impl Noise) {
    noisy.make_noize();
}

fn go(racer: impl Racer) {
    if racer.is_ready() {
        println!("Start.");
    } else {
        println!("stopping");
    }
}

// ❌ TODO: return trait closures
//fn return_racer() -> impl Racer::is_ready() -> bool {
//    || -> bool { true }
//}
fn main() {
    hello(Human {});
    hello(Doc {});
}
