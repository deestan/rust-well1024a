# prng_well1024a

Implementation of the [WELL-1024a](http://www.iro.umontreal.ca/~panneton/WELLRNG.html) pseudorandom number generation algorithm.

In haphazard testing, this generator has proven to be very fast
and provide good random numbers.

From the abstract:

> Fast uniform random number generators with extremely long periods
> have been defined and implemented based on linear recurrences modulo
> 2. The twisted GFSR and the Mersenne twister are famous recent
> examples. Besides the period length, the statistical quality of
> these generators is usually assessed via their equidistribution
> properties. The huge-period generators proposed so far are not quite
> optimal in that respect. In this paper, we propose new generators,
> with better equidistribution and “bit-mixing” properties for
> equivalent period length and speed. Approximately half of the
> coefficients of the characteristic polynomial of these generators
> are nonzero. The state of our new generators evolves in a more
> chaotic way than for the Mersenne twister. We illustrate how this
> can reduce the impact of persistent dependencies among successive
> output values, which can be observed in certain parts of the period
> of gigantic generators such as the Mersenne twister.

# Quick Example

```rust
extern crate prng_well1024a;
extern crate rand;

use rand::Rng;

fn roll<T: Rng>(rng: &mut T) -> String {
    let options = ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"];
    let result = rng.choose(&options).expect("die fell on floor");
    String::from(*result)
}

fn main() {
    let mut rng = prng_well1024a::Well1024aRng::new();
    println!("{} {} {} {} {}",
             roll(&mut rng),
             roll(&mut rng),
             roll(&mut rng),
             roll(&mut rng),
             roll(&mut rng) );
}
```
