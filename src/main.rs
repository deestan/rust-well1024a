extern crate prng_well1024a;
extern crate rand;

use rand::Rng;

fn roll<T: Rng>(rng: &mut T) -> String {
    let options = ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"];
    let result = rng.choose(&options).expect("die fell on floor");
    String::from(*result)
}

#[allow(dead_code)]
fn main() {
    let seed = rand::thread_rng().next_u32();
    let mut rng = prng_well1024a::Well1024aRng::new(seed);
    println!("{}, {}, {}, {}, {}",
             roll(&mut rng),
             roll(&mut rng),
             roll(&mut rng),
             roll(&mut rng),
             roll(&mut rng) );
}
