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
    state: State,
}

struct State {
    i: usize,
    data: [u32; 32],
}

impl Well1024aRng {
    /// Constructs a new `Well1024aRng`, seeded from a number.
    ///
    /// # Examples
    /// 
    /// ```
    /// use prng_well1024a::Well1024aRng;
    ///
    /// let mut rng = Well1024aRng::seed(49152);
    /// ```
    pub fn new(seed: u32) -> Well1024aRng {
        Well1024aRng::load(
            [ seed +  0, seed +  1, seed +  2, seed +  3,
              seed +  4, seed +  5, seed +  6, seed +  7,
              seed +  8, seed +  9, seed + 10, seed + 11,
              seed + 12, seed + 13, seed + 14, seed + 15,
              seed + 16, seed + 17, seed + 18, seed + 19,
              seed + 20, seed + 21, seed + 22, seed + 23,
              seed + 24, seed + 25, seed + 26, seed + 27,
              seed + 28, seed + 29, seed + 30, seed + 31 ])
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
    /// let mut rng = Well1024aRng::load([
    ///    1,  2,  3,  4,  5,  6,  7,  8,
    ///    9, 10, 11, 12, 13, 14, 15, 16,
    ///   17, 18, 19, 20, 21, 22, 23, 24,
    ///   25, 26, 27, 28, 29, 30, 31, 32
    /// ]);
    /// ```
    pub fn load(state: [u32; 32]) -> Well1024aRng {
        Well1024aRng {
            state: State {
                i: 0,
                data: state
            },
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
    pub fn state(&self) -> [u32; 32] {
        let mut out = [0u32; 32];
        let state = &self.state.data;
        let state_i = self.state.i;
        for i in 0..32 {
            out[i] = state[(i + state_i) & 0x1f];
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
        let state = &mut self.state.data;
        let i = self.state.i;

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

        self.state.i = (i + 31) & 31;
        
        state[self.state.i]
    }
}

#[test]
fn it_works() {
    let mut x = Well1024aRng::new();
    let a = x.next_u32();
    let b = x.next_u32();
    assert!(a != b);
}
