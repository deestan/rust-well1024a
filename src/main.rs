extern crate prng_well1024a;
extern crate rand;

use rand::Rng;

#[allow(dead_code)]
fn main() {
    fn inner<T: Rng>(mut rng: T) {
        let mut i = 1_000;
        loop {
            let options = ["apple", "tails"];
            let result = rng.choose(&options)
                .expect("choice failed");
            println!("{}", result);
            //rng.next_u32();
            i -= 1;
            if i == 0 {
                break;
            }
        }
    }
    inner(prng_well1024a::Well1024aRng::new());
}
