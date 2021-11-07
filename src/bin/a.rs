#![allow(clippy::needless_range_loop, clippy::many_single_char_names)]
use proconio::{input, source::line::LineSource};
use std::{
    collections::{hash_map::RandomState, HashSet},
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
    let mut g = vec![vec![]; input.n];
    let mut indeg = vec![0; input.n];
    for (u, v) in input.uv.iter() {
        g[u - 1].push(v - 1);
        indeg[v - 1] += 1;
    }
    // 0になったらタスク開始可能頂点として扱う
    let mut startable_tasks = (0..n).filter(|x| indeg[*x] == 0).collect::<HashSet<_>>();

    // メンバーには割当タスクがあるかどうかの状態があるので、管理したい
    let mut member_state = vec![0; input.m];

    // メンバーごとに割り当てられているタスクが完了するのに要する見積もり日数を管理したい
    // これt_(i,j)を推定することになるのでは？
    let mut member_require_days = vec![0; input.m];

    // 要求技能レベルについては、一旦考慮しないこととする
    let mut s = [
        vec![0, 7, 28, 1, 11, 10, 9, 7, 4, 1, 6, 4, 2, 12, 9],
        vec![16, 1, 14, 9, 0, 5, 15, 6, 2, 0, 4, 6, 2, 4, 7],
        vec![3, 3, 3, 1, 5, 12, 25, 2, 7, 13, 4, 20, 12, 23, 6],
        vec![13, 11, 6, 3, 11, 27, 3, 9, 8, 5, 10, 4, 2, 2, 16],
        vec![6, 5, 2, 1, 6, 14, 2, 15, 14, 3, 20, 6, 5, 17, 0],
        vec![3, 13, 18, 26, 22, 14, 14, 0, 10, 0, 16, 10, 17, 5, 8],
        vec![18, 11, 8, 7, 7, 9, 15, 12, 28, 4, 4, 1, 0, 3, 8],
        vec![1, 1, 4, 5, 9, 2, 5, 12, 5, 2, 7, 3, 3, 3, 2],
        vec![6, 11, 0, 6, 5, 5, 8, 0, 18, 5, 8, 0, 7, 5, 5],
        vec![19, 19, 6, 16, 9, 20, 13, 5, 14, 16, 9, 16, 5, 2, 12],
        vec![8, 7, 4, 6, 14, 14, 3, 8, 3, 3, 27, 4, 6, 11, 10],
        vec![1, 1, 0, 7, 1, 11, 8, 7, 22, 5, 18, 7, 10, 12, 21],
        vec![12, 13, 1, 12, 13, 5, 25, 23, 22, 9, 23, 8, 3, 14, 21],
        vec![3, 2, 14, 6, 15, 9, 3, 1, 16, 6, 7, 1, 6, 1, 1],
        vec![1, 0, 7, 12, 1, 2, 0, 1, 2, 6, 4, 1, 14, 6, 16],
        vec![0, 9, 4, 4, 1, 6, 7, 5, 1, 8, 14, 5, 15, 1, 1],
        vec![2, 11, 9, 12, 11, 3, 4, 19, 5, 4, 9, 4, 18, 30, 27],
        vec![8, 5, 1, 3, 26, 24, 7, 5, 5, 5, 18, 7, 15, 6, 2],
        vec![5, 4, 1, 3, 3, 9, 0, 4, 1, 11, 4, 6, 10, 6, 6],
        vec![8, 14, 12, 4, 14, 2, 3, 7, 6, 23, 18, 8, 7, 2, 11],
    ];

    for s_row in s.iter_mut() {
        if k > s_row.len() {
            let p = k - s_row.len();
            for _ in 0..p {
                s_row.push(5);
            }
        } else {
            let m = s_row.len() - k;
            for _ in 0..m {
                s_row.pop();
            }
        }
    }

    let mut weight = vec![vec![0; input.m]; input.n];
    for (i, w_row) in weight.iter_mut().enumerate() {
        for (j, w) in w_row.iter_mut().enumerate() {
            for k in 0..input.k {
                *w += (input.d[i][k] - s[j][k]).max(0);
            }
            if *w != 0 {
                *w += 3;
            }
            *w = 1.max(*w);
        }
    }

    loop {
        let assign = assign_task(
            &input,
            &startable_tasks,
            &member_state,
            &mut member_require_days,
            &weight,
        );

        // printは一回にまとめた方が早くなる？
        let mut output = assign.len().to_string();
        for (a, b) in assign {
            // memberに割り当てたタスク番号を入れる
            member_state[a] = b + 1;
            startable_tasks.remove(&b);
            output += format!(" {} {}", a + 1, b + 1).as_str();
        }
        println!("{}", output);

        for r_day in member_require_days.iter_mut() {
            if *r_day > 0 {
                *r_day -= 1;
            }
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
            member_state[finished_member - 1] = 0;
            member_require_days[finished_member - 1] = 0;
        }

        // 技能レベルの予測値
        // for (index, line) in s.iter().enumerate() {
        //     print!("#s {}", index + 1);
        //     for predict in line {
        //         print!(" {}", predict);
        //     }
        //     println!();
        // }

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
    weight: &[Vec<i32>],
) -> Vec<(usize, usize)> {
    // 推定しているsを元に割り当てるタスクの割当結果を返す
    // starttable_taskに存在するタスクそれぞれについて、現在フリーなメンバーをw_(i,j)の昇順にソートする
    // starttable_taskについて、w_(i,j) + (取り組んでいるタスクが終わるまでの日数)の合計が小さくなるようにタスクを割り当てる

    let mut fm = (0..input.m).collect::<Vec<_>>();
    let mut assign = vec![];
    for task in startable_tasks.iter() {
        fm.sort_by_key(|j| -(weight[*task][*j] + member_require_days[*j]));
        if let Some(assigned_m) = fm.pop() {
            if member_state[assigned_m] == 0 {
                member_require_days[assigned_m] = weight[*task][assigned_m];
                assign.push((assigned_m, *task));
            }
        }
    }
    assign
}
