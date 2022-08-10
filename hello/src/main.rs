struct Dove;
struct Duck;

trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uooooooohhh!!!!!");
    }
}

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

fn main() {
    fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
        (t, s)
    }

    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Hello", "world");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    let t4 = make_tuple(3, "years old");
}
