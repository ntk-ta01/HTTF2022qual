#![allow(clippy::needless_range_loop, clippy::many_single_char_names)]
use num_integer::sqrt;
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
    // let mut s = [
    //     vec![0, 7, 28, 1, 11, 10, 9, 7, 4, 1, 6, 4, 2, 12, 9],
    //     vec![16, 1, 14, 9, 0, 5, 15, 6, 2, 0, 4, 6, 2, 4, 7],
    //     vec![3, 3, 3, 1, 5, 12, 25, 2, 7, 13, 4, 20, 12, 23, 6],
    //     vec![13, 11, 6, 3, 11, 27, 3, 9, 8, 5, 10, 4, 2, 2, 16],
    //     vec![6, 5, 2, 1, 6, 14, 2, 15, 14, 3, 20, 6, 5, 17, 0],
    //     vec![3, 13, 18, 26, 22, 14, 14, 0, 10, 0, 16, 10, 17, 5, 8],
    //     vec![18, 11, 8, 7, 7, 9, 15, 12, 28, 4, 4, 1, 0, 3, 8],
    //     vec![1, 1, 4, 5, 9, 2, 5, 12, 5, 2, 7, 3, 3, 3, 2],
    //     vec![6, 11, 0, 6, 5, 5, 8, 0, 18, 5, 8, 0, 7, 5, 5],
    //     vec![19, 19, 6, 16, 9, 20, 13, 5, 14, 16, 9, 16, 5, 2, 12],
    //     vec![8, 7, 4, 6, 14, 14, 3, 8, 3, 3, 27, 4, 6, 11, 10],
    //     vec![1, 1, 0, 7, 1, 11, 8, 7, 22, 5, 18, 7, 10, 12, 21],
    //     vec![12, 13, 1, 12, 13, 5, 25, 23, 22, 9, 23, 8, 3, 14, 21],
    //     vec![3, 2, 14, 6, 15, 9, 3, 1, 16, 6, 7, 1, 6, 1, 1],
    //     vec![1, 0, 7, 12, 1, 2, 0, 1, 2, 6, 4, 1, 14, 6, 16],
    //     vec![0, 9, 4, 4, 1, 6, 7, 5, 1, 8, 14, 5, 15, 1, 1],
    //     vec![2, 11, 9, 12, 11, 3, 4, 19, 5, 4, 9, 4, 18, 30, 27],
    //     vec![8, 5, 1, 3, 26, 24, 7, 5, 5, 5, 18, 7, 15, 6, 2],
    //     vec![5, 4, 1, 3, 3, 9, 0, 4, 1, 11, 4, 6, 10, 6, 6],
    //     vec![8, 14, 12, 4, 14, 2, 3, 7, 6, 23, 18, 8, 7, 2, 11],
    // ];

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

            // s[finished_member - 1]に修正をかけたい
            // なんかモデルが必要そう
            let task = member_state[finished_member - 1] - 1;
            processing_tasks.remove(&task);
            member_assigned_tasks[finished_member - 1].push(task);

            member_assigned_tasks[finished_member - 1].sort_by_key(|k| task_time[*k]);

            // task_timeが小さいタスクのdに近くなるようにして、大きいタスクから遠くなるようなベクトルs_jにする

            // for past_task in member_assigned_tasks[finished_member - 1] {
            // 過去のタスクについて、かかった日数がわかる
            // 現在の推定sからかかる日数を推定できる
            // 過去の推定sからかかる日数を推定してある
            // task_tima[past_task] == 1だったらそれ以上の技能がどの技能kについてもある
            // そうでなくとも(かかった日数 - 1)だけ引いた分はどの技能kについても保証できる
            // let pena = task_time[*past_task] - 1;
            // for (k, k_skill) in input.d[*past_task].iter().enumerate() {
            //     let g_skill = (*k_skill - pena * pena * pena).max(*k_skill);
            //     s[finished_member - 1][k] = g_skill;
            // }
            // eprintln!("{:?}", s[finished_member - 1]);
            for past_task in member_assigned_tasks[finished_member - 1].iter() {
                let mut w = 0;
                for k in 0..input.k {
                    w += (input.d[*past_task][k] - s[finished_member - 1][k]).max(0);
                }
                w = 1.max(w);
                weight.insert((*past_task, finished_member - 1), w);

                // 直前のタスクの影響を受けすぎる
                // それまでのタスクをすべて考慮したい
                let past_task_l2 = sqrt(input.d[task].iter().map(|x| *x * *x).sum::<i32>());
                let s_j_l2 = sqrt(s[finished_member - 1].iter().map(|x| *x * *x).sum::<i32>());
                if weight[&(*past_task, finished_member - 1)] < task_time[*past_task] {
                    for (d_k, s_k) in input.d[*past_task]
                        .iter()
                        .zip(s[finished_member - 1].iter_mut())
                    {
                        if *d_k < 6 && 6 <= *s_k {
                            *s_k = *d_k;
                        } else if past_task_l2 - s_j_l2 > input.k as i32 {
                            *s_k = 0.max(*s_k - (past_task_l2 - s_j_l2) / input.k as i32);
                        } else if past_task_l2 - s_j_l2 > 6 as i32 && 13 <= *s_k {
                            *s_k = 0.max(*s_k - 3);
                        }
                    }
                }
                if weight[&(*past_task, finished_member - 1)] > task_time[*past_task] {
                    for (d_k, s_k) in input.d[*past_task]
                        .iter()
                        .zip(s[finished_member - 1].iter_mut())
                    {
                        if *s_k < *d_k {
                            *s_k = *d_k;
                        }
                        if past_task_l2 - s_j_l2 > input.k as i32 {
                            *s_k += (past_task_l2 - s_j_l2) / input.k as i32;
                        }
                        // *d_k のl2ノルムがある程度大きかったら*s_kのl2ノルムも計算して小さかったら少し足す
                    }
                }
            }
            // }
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
            // for i in 0..input.m {
            //     member_assigned_tasks[i].sort_by_key(|k| task_time[*k]);
            //     eprint!("{}: ", i);
            //     for x in member_assigned_tasks[i].iter().take(10) {
            //         eprint!("{} ", task_time[*x]);
            //     }
            //     eprint!("/ ");
            //     for x in member_assigned_tasks[i].iter().rev().take(10).rev() {
            //         eprint!("{} ", task_time[*x]);
            //     }
            //     eprintln!();
            // }
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
    // 推定しているsを元に割り当てるタスクの割当結果を返す
    // starttable_taskに存在するタスクそれぞれについて、現在フリーなメンバーをw_(i,j)の昇順にソートする
    // starttable_taskについて、w_(i,j) + (取り組んでいるタスクが終わるまでの日数)の合計が小さくなるようにタスクを割り当てる

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
        fm.sort_by_key(|j| {
            Reverse(weight[&(task, *j)] + member_require_days[*j])
            // + if member_state[*j] > 0 { 50 } else { 0 })
        });
        if let Some(assigned_m) = fm.pop() {
            if member_state[assigned_m] == 0 {
                member_require_days[assigned_m] = weight[&(task, assigned_m)];
                assign.push((assigned_m, task));
            }
        }
    }
    assign
}
