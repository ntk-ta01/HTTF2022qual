#![allow(clippy::needless_range_loop, clippy::many_single_char_names)]

use proconio::{input, source::line::LineSource};
// use rand::prelude::*;
use std::{
    cmp::Reverse,
    collections::{hash_map::RandomState, HashMap, HashSet},
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

    // let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);

    // タスクには依存関係があるので、始められるタスクを列挙したい
    // 有向グラフだと思って、入次数が0の点から始めるのがよさそう？
    // タスクが完了したらタスク頂点から伸びている他の頂点の入次数のそれぞれ-1
    // 出次数が0のタスクは後回しでよい
    let mut g = vec![vec![]; input.n];
    let mut indeg = vec![0; input.n];
    let mut outdeg = vec![0; input.n];
    for (u, v) in input.uv.iter() {
        g[u - 1].push(v - 1);
        indeg[v - 1] += 1;
        outdeg[u - 1] += 1;
    }
    // 0になったらタスク開始可能頂点として扱う
    let mut startable_tasks = (0..n).filter(|x| indeg[*x] == 0).collect::<HashSet<_>>();
    // 進行中のタスク
    let mut processing_tasks = HashSet::new();

    // メンバーには割当タスクがあるかどうかの状態があるので、管理したい
    let mut member_state = vec![0; input.m];

    // メンバーごとに割り当てられているタスクが完了するのに要する見積もり日数を管理したい
    // マイナスなら想定より遅く終わっている
    let mut member_require_days = vec![0; input.m];
    // メンバーごとに過去に割り振られたタスクと、かかった日数を覚えておく
    let mut member_assigned_tasks = vec![vec![]; input.m];
    let mut task_time = vec![-1; input.n];

    // 要求技能レベル
    let mut s = vec![vec![0; input.k]; input.m];
    // for i in 0..input.m {
    //     let mut b = vec![0.0; input.k];
    //     for j in 0..input.k {
    //         b[j] = f64::abs(rng.sample(rand_distr::StandardNormal));
    //     }
    //     let mul = rng.gen_range(20.0, 60.0) / b.iter().map(|x| x * x).sum::<f64>().sqrt();
    //     for j in 0..input.k {
    //         s[i][j] = (b[j] * mul).round() as i32;
    //     }
    // }

    let mut weight = HashMap::new();

    // let mut day = 0;
    loop {
        // day += 1;
        // eprintln!("day: {}", day);
        let assign = assign_task(
            &input,
            &startable_tasks,
            &member_state,
            &mut member_require_days,
            &s,
            &outdeg,
            &mut weight,
        );
        // printは一回にまとめた方が早くなる？
        let mut output = assign.len().to_string();
        for (a, b) in assign {
            // memberに割り当てたタスク番号を入れる
            member_state[a] = b + 1;
            processing_tasks.insert(b);
            task_time[b] = 0;
            startable_tasks.remove(&b);
            output += format!(" {} {}", a + 1, b + 1).as_str();
        }
        println!("{}", output);

        for r_day in member_require_days.iter_mut() {
            *r_day -= 1;
        }

        for task in processing_tasks.iter() {
            task_time[*task] += 1;
        }

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

            let task = member_state[finished_member - 1] - 1;
            processing_tasks.remove(&task);
            member_assigned_tasks[finished_member - 1].push(task);

            member_require_days[finished_member - 1] = 0;
            member_state[finished_member - 1] = 0;
        }

        // 技能レベルの予測値
        for (index, line) in s.iter().enumerate() {
            print!("#s {}", index + 1);
            for predict in line {
                print!(" {}", predict);
            }
            println!();
        }

        if n == -1 {
            break;
        }
    }
}

fn assign_task(
    input: &Input,
    startable_tasks: &HashSet<usize, RandomState>,
    member_state: &[usize],
    member_require_days: &mut Vec<i32>,
    s: &[Vec<i32>],
    outdeg: &[i32],
    weight: &mut HashMap<(usize, usize), i32>,
) -> Vec<(usize, usize)> {
    let mut fm = (0..input.m).collect::<Vec<_>>();
    let mut assign = vec![];
    let mut sorted_tasks = startable_tasks.clone().into_iter().collect::<Vec<_>>();
    sorted_tasks.sort_by_key(|task| Reverse(outdeg[*task]));
    for task in sorted_tasks {
        for j in fm.iter() {
            let mut w = 0;
            for k in 0..input.k {
                w += (input.d[task][k] - s[*j][k]).max(0);
            }
            w = 1.max(w);
            let key = (task, *j);
            weight.insert(key, w);
        }
        fm.sort_by_key(|j| Reverse(weight[&(task, *j)] + member_require_days[*j]));
        if let Some(assigned_m) = fm.pop() {
            if member_state[assigned_m] == 0 {
                member_require_days[assigned_m] = weight[&(task, assigned_m)];
                assign.push((assigned_m, task));
            }
        }
    }
    assign
}
