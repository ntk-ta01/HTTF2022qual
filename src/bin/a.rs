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
#[derive(Debug, Clone, Copy)]
struct Member {
    assigned: usize, // 割り当てられたタスク番号
    started: i64,    // タスクの開始日
}

impl Member {
    fn new(input: &Input) -> Self {
        let assigned = input.n;
        let started = -1;
        Member { assigned, started }
    }
}

fn main() {
    // phocomさんの解法を勉強する
    // https://twitter.com/_phocom/status/1459462132916195329
    // https://atcoder.jp/contests/future-contest-2022-qual/submissions/27207347
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

    let skill = vec![vec![5; input.k]; input.m];
    let mut completed = vec![false; input.n]; // タスクについて割り当てたかどうか
    let mut members = vec![Member::new(&input); input.m];

    for turn in 1.. {
        let mut output = vec![];
        // assign tasks
        let mut set_work =
            |task_i: usize, member_i: usize, completed: &mut [bool], members: &mut [Member]| {
                output.push((member_i, task_i));
                completed[task_i] = true;
                members[member_i].assigned = task_i;
                members[member_i].started = turn;
            };
        for i in 0..input.m {
            if members[i].assigned == input.n {
                let mut best = input.n;
                for j in 0..input.n {
                    if !completed[j] && indeg[j] == 0 && best == input.n {
                        // j は割当可能なタスク
                        best = j;
                        break;
                    }
                }
                if best != input.n {
                    set_work(best, i, &mut completed, &mut members);
                }
            }
        }
        // simulation

        // output
        print!("{}", output.len());
        for (member, task) in output {
            print!(" {} {}", member + 1, task + 1);
        }
        println!();
        if turn == 2000 {
            break;
        }
        // input
        input! {
            from &mut source,
            n: usize,
            _: [i64; n],
        }
        // feedback
    }
    input! { from &mut source, _: i64};
}
