//! Implementation of the [WELL-1024a](http://www.iro.umontreal.ca/~panneton/WELLRNG.html) pseudorandom number generation algorithm.
//!
//! Implements the `rand::Rng` trait.
//!
//! From the abstract:
//! 
//! > Fast uniform random number generators with extremely long
//! > periods have been defined and implemented based on linear
//! > recurrences modulo 2. The twisted GFSR and the Mersenne twister
//! > are famous recent examples. Besides the period length, the
//! > statistical quality of these generators is usually assessed via
//! > their equidistribution properties. The huge-period generators
//! > proposed so far are not quite optimal in that respect. In this
//! > paper, we propose new generators, with better equidistribution and
//! > “bit-mixing” properties for equivalent period length and
//! > speed. Approximately half of the coefficients of the
//! > characteristic polynomial of these generators are nonzero. The
//! > state of our new generators evolves in a more chaotic way than for
//! > the Mersenne twister. We illustrate how this can reduce the impact
//! > of persistent dependencies among successive output values, which
//! > can be observed in certain parts of the period of gigantic
//! > generators such as the Mersenne twister.
//!
//! In haphazard testing, this generator has proven to be very fast
//! and provide good random numbers..

extern crate rand;

use rand::Rng;

pub struct Well1024aRng {
    i: usize,
    state: [u32; 32],
}

impl Well1024aRng {
    /// Constructs a new `Well1024aRng`, seeded from rand::thread_rng().
    ///
    /// # Examples
    /// 
    /// ```
    /// use prng_well1024a::Well1024aRng;
    ///
    /// let mut rng = Well1024aRng::new();
    /// ```
    pub fn new() -> Well1024aRng {
        let mut rng = rand::thread_rng();
        let state = (0..32).map(|_| { rng.next_u32() }).collect();
        Well1024aRng::load(state).unwrap()
    }
    /// Constructs a new `Well1024aRng`, seeded from a number.
    ///
    /// # Examples
    /// 
    /// ```
    /// use prng_well1024a::Well1024aRng;
    ///
    /// let mut rng = Well1024aRng::seed(49152);
    /// ```
    pub fn seed(seed: u32) -> Well1024aRng {
        let state = (0..32).map(|i| { i + seed }).collect();
        Well1024aRng::load(state).unwrap()
    }

    /// Constructs a new `Well1024aRng`, from a full 1024-bit state.
    /// The state can be the result of a previous `state()` call,
    /// allowing for exact replay of output.
    ///
    /// # Examples
    /// 
    /// ```
    /// use prng_well1024a::Well1024aRng;
    ///
    /// let mut rng = Well1024aRng::load((0..32).collect());
    /// ```
    pub fn load(state: Vec<u32>) -> Result<Well1024aRng, &'static str> {
        if state.len() != 32 {
            Err("Invalid state: len() != 32")
        } else {
            let mut rng = Well1024aRng { i: 0, state: [0u32; 32] };
            for i in (0..32) {
                rng.state[i] = state[i];
            }
            Ok(rng)
        }
    }

    /// Returns the full 1024-bit state of a `Well1024aRng`.
    /// This can be used to "rewind" the generator by recreating
    /// it from a previous state via `Well1024aRng::load()`.
    ///
    /// # Examples
    /// 
    /// ```
    /// use prng_well1024a::Well1024aRng;
    ///
    /// let mut rng = Well1024aRng::new();
    /// let state = rng.state();
    /// ```
    pub fn state(&self) -> Vec<u32> {
        let mut out = vec![0u32; 32];
        for i in 0..32 {
            let val = self.state[(i + self.i) & 0x1f];
            out.push(val);
        }
        out
    }
}

impl Rng for Well1024aRng {
    fn next_u32(&mut self) -> u32 {
        fn mat0pos(t: i32, v: u32) -> u32 {
            v ^ (v >> t)
        }

        fn mat0neg(t: i32, v: u32) -> u32 {
            v ^ (v << -t)
        }

        let m1 = 3;
        let m2 = 24;
        let m3 = 10;
        let state = &mut self.state;
        let i = self.i;

        let z0 = state[(i + 31) & 31];
        let z1 =
            state[i] ^
            mat0pos(8, state[(i + m1) & 31]);
        let z2 =
            mat0neg(-19, state[(i + m2) & 31]) ^
            mat0neg(-14, state[(i + m3) & 31]);

        state[i] = z1 ^ z2;

        state[(i + 31) & 31] =
            mat0neg(-11, z0) ^
            mat0neg(-7, z1) ^
            mat0neg(-13, z2);

        self.i = (i + 31) & 31;
        
        state[self.i]
    }
}

#[test]
fn it_works() {
    let mut x = Well1024aRng::new();
    let a = x.next_u32();
    let b = x.next_u32();
    assert!(a != b);
}
