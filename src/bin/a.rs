use proconio::{input, marker::Usize1, source::line::LineSource};
use std::io::{self, BufReader};
// use rand::prelude::*;

struct Input {
    n: usize,
    m: usize,
    k: usize,
    r: usize,
    ds: Vec<Vec<i64>>,
    uv: Vec<(usize, usize)>,
}

fn main() {
    let mut source = LineSource::new(BufReader::new(io::stdin()));
    input! {
        from &mut source,
        n: usize,
        m: usize,
        k: usize,
        r: usize,
        ds: [[i64; k]; n],
        uv: [(Usize1, Usize1); r],
    }
    let input = Input { n, m, k, r, ds, uv };
    // let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);

    let mut g = vec![vec![]; input.n];
    let mut indeg = vec![0; input.n];
    for &(u, v) in input.uv.iter() {
        g[u].push(v);
        indeg[v] += 1;
    }

    let skill = vec![vec![0; input.k]; input.m];

    for turn in 1.. {
        println!("{}", 0);
        if turn == 2000 {
            break;
        }
        input! {
            from &mut source,
            n: usize,
            _: [i64; n],
        }
    }
    input! { from &mut source, _: i64};
}
