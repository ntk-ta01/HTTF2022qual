#![allow(clippy::needless_range_loop, clippy::many_single_char_names)]
use im_rc::HashMap;
use proconio::{input, source::line::LineSource};
use rand::prelude::*;
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

    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);

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
    // 進行中のタスク
    let mut processing_tasks = HashSet::new();

    // メンバーには割当タスクがあるかどうかの状態があるので、管理したい
    let mut member_state = vec![0; input.m];

    // メンバーごとに割り当てられているタスクが完了するのに要する見積もり日数を管理したい
    // マイナスなら想定より遅く終わっている
    let mut member_require_days = vec![0; input.m];
    // これt_(i,j)を推定することになるのでは？
    // メンバーごとに過去に割り振られたタスクと、かかった日数を覚えておく？
    // かかった日数の少ないタスクの要求技能レベルをメンバーjの技能だとする
    let mut member_assigned_tasks = vec![vec![]; input.m];
    let mut task_time = vec![-1; input.n];

    // 要求技能レベル
    let mut s = vec![vec![1; input.k]; input.m];
    for i in 0..input.m {
        let mut b = vec![0.0; input.k];
        for j in 0..input.k {
            b[j] = f64::abs(rng.sample(rand_distr::StandardNormal));
        }
        let mul = rng.gen_range(20.0, 60.0) / b.iter().map(|x| x * x).sum::<f64>().sqrt();
        for j in 0..input.k {
            s[i][j] = (b[j] * mul).round() as i32;
        }
    }

    loop {
        let assign = assign_task(
            &input,
            &startable_tasks,
            &member_state,
            &mut member_require_days,
            &s,
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
            // let task = member_state[finished_member - 1]; d[task - 1]について、遅れ日数分を引いた能力については保証できる
            // 実際にはそこまで悲観的でなくともよさそう？ もう少し技能レベルについて楽観的に見てもよい？
            // なんかモデルが必要そう
            let task = member_state[finished_member - 1] - 1;
            processing_tasks.remove(&task);
            member_assigned_tasks[finished_member - 1].push(task);

            member_assigned_tasks[finished_member - 1].sort_by_key(|k| task_time[*k]);

            // task_timeが小さいタスクのdに近くなるようにして、大きいタスクから遠くなるようなベクトルs_jにする
            s[finished_member - 1] =
                input.d[member_assigned_tasks[finished_member - 1][0] as usize].clone();

            for past_task in member_assigned_tasks[finished_member - 1].iter() {
                if task_time[*past_task] == 1 {
                    for (k, k_skill) in input.d[*past_task].iter().enumerate() {
                        s[finished_member - 1][k] = s[finished_member - 1][k].max(*k_skill);
                    }
                } else {
                    break;
                }
            }

            // for (required_skill, skill) in
            //     input.d[task].iter().zip(s[finished_member - 1].iter_mut())
            // {
            // 想定より早く終わっていたら+、想定より遅く終わっていたら-の値
            // だと思ったが、これでは一度本来の技能よりも大きくなると想定よりも早く終わって技能が本来の技能よりも大きくなってしまう

            // *skill = (required_skill + delay / input.k as i32 / 2).max(*skill);
            // ↑require_skillに足してskillと大きい方だと技能kごとには変化しない
            // delayの大きさとrequired_skillの大きさに応じて、skillがいい感じに変化していくと嬉しい
            // s_i,jの生成方法を真似するとよい？参考にする必要はありそう
            // s_iの合計値はどれくらいになる？
            // s_i,jは[0,60]くらい
            // *skill = (required_skill + delay + *skill) / 2;
            // ↑delayの絶対値が小さいほどrequired_skillに似ている
            // *skill = *required_skill;
            // あとweightも変えなきゃいけないよな
            // タスク遂行に何日かかったか、required_skill、現在の推定skill
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
            //     eprint!("{}: ", i);
            //     for x in member_assigned_tasks[i].iter().take(5) {
            //         eprint!("{} ", task_time[*x]);
            //     }
            //     eprint!("/ ");
            //     for x in member_assigned_tasks[i].iter().rev().take(5) {
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
) -> Vec<(usize, usize)> {
    // 推定しているsを元に割り当てるタスクの割当結果を返す
    // starttable_taskに存在するタスクそれぞれについて、現在フリーなメンバーをw_(i,j)の昇順にソートする
    // starttable_taskについて、w_(i,j) + (取り組んでいるタスクが終わるまでの日数)の合計が小さくなるようにタスクを割り当てる

    let mut fm = (0..input.m).collect::<Vec<_>>();
    let mut assign = vec![];
    let mut weight: HashMap<usize, i32> = HashMap::new();
    for task in startable_tasks.iter() {
        for j in fm.iter() {
            let mut w = 0;
            for k in 0..input.k {
                w += (input.d[*task][k] - s[*j][k]).max(0);
            }
            if w != 0 {
                w += 3;
            }
            w = 1.max(w) - 1;
            let key = *task + *j * input.n;
            weight.insert(key, w);
        }
        fm.sort_by_key(|j| -(weight[&(*task + *j * input.n)] + member_require_days[*j]));
        if let Some(assigned_m) = fm.pop() {
            if member_state[assigned_m] == 0 {
                member_require_days[assigned_m] = weight[&(*task + assigned_m * input.n)];
                assign.push((assigned_m, *task));
            }
        }
    }
    assign
}
