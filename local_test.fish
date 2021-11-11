set score 0
set now 0
cargo run --release -p tools --bin tester cargo run --release --bin future-contest-2022-qual-a < ./in/0000.txt > out.txt ^tmp
cat tmp | while read line
    set now $line
end
echo 0 $now
set score (math $score + $now)

for val in (seq 1 9)
  cargo run --release -p tools --bin tester cargo run --release --bin future-contest-2022-qual-a < ./in/000$val.txt >./out/000$val.txt ^tmp
  cat tmp | while read line
    set now $line
  end
  echo $val $now
  set score (math $score + $now)
end

for val in (seq 10 99)
  cargo run --release -p tools --bin tester cargo run --release --bin future-contest-2022-qual-a < ./in/00$val.txt >./out/00$val.txt ^tmp
  cat tmp | while read line
    set now $line
  end
  echo $val $now
  set score (math $score + $now)
end
set score (math $score / 2)
echo score: $score
rm tmp