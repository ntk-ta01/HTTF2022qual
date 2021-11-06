#![allow(clippy::needless_range_loop, clippy::many_single_char_names)]
use proconio::{input, source::line::LineSource};
use std::{
    collections::HashSet,
    io::{self, BufReader},
};

#[allow(dead_code)]
struct Input {
    n: usize,
    m: usize,
    k: usize,
    r: usize,
    d: Vec<Vec<i32>>,
    uv: Vec<(usize, usize)>,
}

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n: usize,
        m: usize,
        k: usize,
        r: usize,
        d: [[i32; k]; n],
        uv: [(usize, usize); r],
    }
    let input = Input { n, m, k, r, d, uv };

    // タスクには依存関係があるので、始められるタスクを列挙したい
    // 有向グラフだと思って、入次数が0の点から始めるのがよさそう？
    // タスクが完了したらタスク頂点から伸びている他の頂点の入次数のそれぞれ-1
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for (u, v) in input.uv.iter() {
        g[u - 1].push(v - 1);
        indeg[v - 1] += 1;
    }
    // 0になったらタスク開始可能頂点として扱う
    let mut startable_tasks = (0..n).filter(|x| indeg[*x] == 0).collect::<HashSet<_>>();

    // メンバーには割当タスクがあるかどうかの状態があるので、管理したい
    let mut member_state = vec![0; m];

    // 要求技能レベルについては、一旦考慮しないこととする

    loop {
        // メンバーに対して割り当てられるタスクをすべて割り当てる
        let free_members = member_state
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == 0)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        let m = free_members.len().min(startable_tasks.len());
        let assign = free_members
            .iter()
            .zip(startable_tasks.clone())
            .collect::<Vec<_>>();

        // printは一回にまとめた方が早くなる？
        print!("{}", m);
        for (a, b) in assign {
            // memberに割り当てたタスク番号を入れる
            member_state[*a] = b + 1;
            startable_tasks.remove(&b);
            print!(" {} {}", a + 1, b + 1);
        }
        println!();

        let (n, f) = {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_ascii_whitespace();
            let n = iter.next().unwrap().parse::<i32>().unwrap();
            (n, iter.map(|s| s.parse().unwrap()).collect::<Vec<usize>>())
        };

        for finished_member in f {
            let finished_task_number = member_state[finished_member - 1] - 1;
            for nb in g[finished_task_number].iter() {
                indeg[*nb] -= 1;
                if indeg[*nb] == 0 {
                    startable_tasks.insert(*nb);
                }
            }
            member_state[finished_member - 1] = 0;
        }

        if n == -1 {
            break;
        }
    }
}
