use super::State;

pub(crate) fn theta(state: &mut State) {
    let mut c = [0u64; 5];
    for i in 0..5 {
        c[i] = state[i][0];
        for j in 1..5 {
            c[i] ^= state[i][j];
        }
    }

    let mut d = [0u64; 5];
    for i in 0..5 {
        let l = if i == 0 { 4 } else { i - 1 };
        let r = (i + 1) % 5;
        d[i] = c[l] ^ (c[r].rotate_left(1))
    }

    for x in 0..5 {
        for y in 0..5 {
            state[x][y] ^= d[x];
        }
    }
}

pub(crate) fn rho(state: &mut State) {
    let (mut x, mut y) = (1, 0);
    for t in 0..24 {
        let offset = (((t + 1) * (t + 2)) / 2) % 64;
        state[x][y] = state[x][y].rotate_left(offset);
        (x, y) = (y, ((2 * x) + (3 * y)) % 5);
    }
}

pub(crate) fn pi(state: &mut State) {
    let mut out = State::new();
    for x in 0..5 {
        for y in 0..5 {
            out[x][y] = state[(x + 3 * y) % 5][x];
        }
    }
    *state = out;
}

pub(crate) fn chi(state: &mut State) {
    let mut out = State::new();
    for x in 0..5 {
        for y in 0..5 {
            out[x][y] = state[x][y] ^ (!state[(x + 1) % 5][y] & state[(x + 2) % 5][y]);
        }
    }
    *state = out;
}

pub(crate) fn iota(round_num: u8, state: &mut State) {
    state[0][0] ^= rc(round_num);
}

fn lfsr(t: u8) -> bool {
    if t == 0 {
        return true;
    }
    let mut state = 0x01u8;
    for _ in 0..t {
        state = if state & 0x80 != 0 {
            (state << 1) ^ 0x71
        } else {
            state << 1
        };
    }
    state & 0x01 == 1
}

fn rc(round_num: u8) -> u64 {
    let mut out = 0;
    for j in 0..7 {
        if lfsr(j + 7 * round_num) {
            out |= 1 << ((1 << j) - 1)
        }
    }
    if round_num == 1 {
        println!("{:x}", out)
    }
    out
}

#[cfg(test)]
mod test {

    #[test]
    fn round_constants() {
        let rcs = vec![
            0x0000000000000001,
            0x0000000000008082,
            0x800000000000808a,
            0x8000000080008000,
            0x000000000000808b,
            0x0000000080000001,
            0x8000000080008081,
            0x8000000000008009,
            0x000000000000008a,
            0x0000000000000088,
            0x0000000080008009,
            0x000000008000000a,
            0x000000008000808b,
            0x800000000000008b,
            0x8000000000008089,
            0x8000000000008003,
            0x8000000000008002,
            0x8000000000000080,
            0x000000000000800a,
            0x800000008000000a,
            0x8000000080008081,
            0x8000000000008080,
            0x0000000080000001,
            0x8000000080008008,
        ];
        assert_eq!(
            (0..24)
                .into_iter()
                .map(|r| super::rc(r))
                .collect::<Vec<_>>(),
            rcs
        );
    }
}
