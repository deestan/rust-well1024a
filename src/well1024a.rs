pub struct Well1024aRng {
    state: State,
}

pub struct State {
    i: usize,
    data: [u32; 32],
}

impl Well1024aRng {
    pub fn new() -> Well1024aRng {
        Well1024aRng {
            state: State {
                i: 0,
                data: [ 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1, 1 ]
            },
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        fn mat0pos(t: i32, v: u32) -> u32 {
            v ^ (v >> t)
        }

        fn mat0neg(t: i32, v: u32) -> u32 {
            v ^ (v << (-t))
        }

        let m1 = 3;
        let m2 = 24;
        let m3 = 10;
        let state = &mut self.state.data;
        let i = &mut self.state.i;

        let z0 = state[(*i + 31) & 31];
        let z1 =
            state[*i] ^
            mat0pos(8, state[(*i + m1) & 31]);
        let z2 =
            mat0neg(-19, state[(*i + m2) & 31]) ^
            mat0neg(-14, state[(*i + m3) & 31]);

        state[*i] = z1 ^ z2;

        state[(*i + 31) & 31] =
            mat0neg(-11, z0) ^
            mat0neg(-7, z1) ^
            mat0neg(-13, z2);

        *i = (*i + 31) & 31;
        
        state[*i]
    }
}

#[test]
fn it_works() {
    let mut x = Well1024aRng::new();
    let a = x.next_u32();
    let b = x.next_u32();
    assert!(a != b);
}
