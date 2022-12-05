use std::collections::HashMap;

use regex::Regex;

type Piles = HashMap<usize, Vec<char>>;

fn move_items(piles: &mut Piles, count: usize, from: usize, to: usize) -> Piles {
    println!("move_items: {count}, {from} -> {to}");
    let mut from_vec = piles.get(&(from)).unwrap().to_vec();
    let mut to_vec = piles.get(&(to)).unwrap().to_vec();
    for _ in 0..count {
        to_vec.push(from_vec.pop().unwrap());
    }
    piles.insert(from, from_vec);
    piles.insert(to, to_vec);
    piles.to_owned()
}

fn main() {
    //     let data = "    [D]
    // [N] [C]
    // [Z] [M] [P]";
    let data = "
    [G] [R]                 [P]
    [H] [W]     [T] [P]     [H]
    [F] [T] [P] [B] [D]     [N]
[L] [T] [M] [Q] [L] [C]     [Z]
[C] [C] [N] [V] [S] [H]     [V] [G]
[G] [L] [F] [D] [M] [V] [T] [J] [H]
[M] [D] [J] [F] [F] [N] [C] [S] [F]
[Q] [R] [V] [J] [N] [R] [H] [G] [Z]";

    // let move_input = "move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    // move 1 from 1 to 2";

    let move_input = "move 5 from 8 to 2
    move 2 from 4 to 5
    move 3 from 3 to 9
    move 4 from 1 to 8
    move 5 from 9 to 1
    move 3 from 3 to 8
    move 2 from 4 to 7
    move 6 from 6 to 5
    move 5 from 2 to 4
    move 2 from 9 to 1
    move 1 from 7 to 1
    move 4 from 7 to 3
    move 5 from 1 to 5
    move 3 from 1 to 4
    move 8 from 5 to 3
    move 7 from 3 to 2
    move 10 from 4 to 7
    move 1 from 7 to 3
    move 1 from 6 to 2
    move 3 from 8 to 4
    move 4 from 3 to 2
    move 1 from 1 to 2
    move 4 from 3 to 1
    move 2 from 1 to 7
    move 3 from 5 to 1
    move 7 from 8 to 4
    move 9 from 5 to 1
    move 9 from 2 to 7
    move 6 from 4 to 9
    move 14 from 7 to 5
    move 2 from 1 to 4
    move 6 from 7 to 1
    move 4 from 4 to 9
    move 6 from 2 to 8
    move 2 from 4 to 9
    move 2 from 9 to 3
    move 3 from 8 to 3
    move 5 from 9 to 4
    move 1 from 2 to 9
    move 5 from 5 to 3
    move 3 from 2 to 7
    move 1 from 1 to 4
    move 3 from 7 to 5
    move 4 from 9 to 6
    move 2 from 9 to 3
    move 5 from 1 to 6
    move 7 from 6 to 5
    move 1 from 2 to 3
    move 10 from 1 to 5
    move 1 from 8 to 3
    move 14 from 3 to 7
    move 1 from 8 to 4
    move 2 from 6 to 1
    move 28 from 5 to 9
    move 1 from 2 to 1
    move 5 from 4 to 6
    move 2 from 4 to 3
    move 13 from 7 to 8
    move 1 from 3 to 5
    move 1 from 5 to 2
    move 1 from 3 to 6
    move 1 from 5 to 6
    move 22 from 9 to 1
    move 1 from 2 to 7
    move 3 from 9 to 5
    move 2 from 7 to 5
    move 18 from 1 to 4
    move 7 from 8 to 3
    move 4 from 6 to 8
    move 2 from 5 to 8
    move 5 from 3 to 9
    move 2 from 5 to 1
    move 3 from 6 to 8
    move 1 from 5 to 9
    move 2 from 3 to 6
    move 10 from 1 to 5
    move 15 from 8 to 6
    move 10 from 6 to 8
    move 1 from 9 to 4
    move 1 from 1 to 3
    move 4 from 4 to 3
    move 5 from 3 to 5
    move 9 from 5 to 6
    move 13 from 6 to 5
    move 8 from 5 to 7
    move 8 from 9 to 6
    move 2 from 6 to 4
    move 2 from 6 to 2
    move 3 from 7 to 4
    move 2 from 2 to 8
    move 1 from 5 to 4
    move 3 from 7 to 9
    move 1 from 5 to 9
    move 5 from 6 to 9
    move 10 from 8 to 3
    move 3 from 8 to 1
    move 5 from 9 to 2
    move 1 from 6 to 4
    move 4 from 5 to 6
    move 7 from 3 to 7
    move 5 from 6 to 5
    move 19 from 4 to 8
    move 15 from 8 to 3
    move 2 from 1 to 5
    move 7 from 5 to 9
    move 2 from 7 to 2
    move 3 from 3 to 8
    move 5 from 5 to 8
    move 10 from 9 to 3
    move 1 from 4 to 2
    move 10 from 8 to 3
    move 29 from 3 to 2
    move 2 from 3 to 4
    move 1 from 1 to 5
    move 2 from 8 to 4
    move 1 from 9 to 1
    move 1 from 3 to 9
    move 1 from 1 to 9
    move 2 from 3 to 4
    move 33 from 2 to 1
    move 2 from 2 to 4
    move 1 from 3 to 1
    move 22 from 1 to 2
    move 6 from 4 to 9
    move 4 from 7 to 1
    move 16 from 1 to 4
    move 3 from 7 to 6
    move 2 from 9 to 4
    move 1 from 5 to 2
    move 9 from 4 to 2
    move 1 from 6 to 5
    move 7 from 4 to 2
    move 6 from 9 to 8
    move 4 from 4 to 9
    move 4 from 8 to 3
    move 2 from 4 to 3
    move 2 from 2 to 5
    move 2 from 5 to 2
    move 1 from 5 to 6
    move 3 from 9 to 5
    move 1 from 6 to 8
    move 2 from 6 to 5
    move 1 from 3 to 2
    move 1 from 8 to 4
    move 2 from 8 to 2
    move 5 from 5 to 6
    move 44 from 2 to 8
    move 1 from 4 to 8
    move 3 from 6 to 8
    move 2 from 6 to 2
    move 37 from 8 to 3
    move 1 from 9 to 4
    move 1 from 2 to 5
    move 5 from 8 to 6
    move 1 from 4 to 6
    move 1 from 2 to 4
    move 16 from 3 to 2
    move 1 from 4 to 5
    move 1 from 8 to 3
    move 4 from 8 to 2
    move 1 from 8 to 7
    move 2 from 5 to 8
    move 15 from 2 to 4
    move 5 from 6 to 3
    move 1 from 7 to 4
    move 1 from 8 to 9
    move 1 from 6 to 7
    move 1 from 8 to 3
    move 2 from 2 to 8
    move 1 from 9 to 3
    move 2 from 8 to 4
    move 1 from 4 to 6
    move 33 from 3 to 7
    move 1 from 6 to 3
    move 1 from 4 to 8
    move 1 from 8 to 9
    move 4 from 4 to 3
    move 9 from 4 to 7
    move 3 from 4 to 8
    move 11 from 7 to 2
    move 14 from 7 to 6
    move 1 from 8 to 3
    move 1 from 9 to 5
    move 1 from 5 to 1
    move 8 from 2 to 9
    move 1 from 8 to 7
    move 6 from 3 to 6
    move 18 from 6 to 4
    move 1 from 2 to 7
    move 1 from 3 to 6
    move 14 from 4 to 2
    move 4 from 4 to 3
    move 3 from 6 to 3
    move 19 from 2 to 6
    move 16 from 6 to 8
    move 1 from 1 to 8
    move 16 from 8 to 7
    move 3 from 9 to 4
    move 3 from 6 to 2
    move 3 from 4 to 7
    move 4 from 3 to 2
    move 2 from 2 to 4
    move 4 from 9 to 8
    move 5 from 2 to 8
    move 29 from 7 to 5
    move 6 from 8 to 2
    move 2 from 3 to 4
    move 2 from 2 to 6
    move 1 from 3 to 5
    move 4 from 2 to 6
    move 8 from 7 to 5
    move 1 from 7 to 5
    move 2 from 8 to 6
    move 1 from 8 to 7
    move 6 from 6 to 1
    move 2 from 7 to 6
    move 1 from 9 to 7
    move 3 from 1 to 7
    move 3 from 6 to 1
    move 1 from 7 to 6
    move 3 from 1 to 6
    move 1 from 1 to 5
    move 4 from 6 to 3
    move 2 from 4 to 2
    move 38 from 5 to 6
    move 3 from 3 to 8
    move 4 from 8 to 6
    move 22 from 6 to 8
    move 1 from 7 to 8
    move 2 from 6 to 2
    move 2 from 5 to 2
    move 2 from 2 to 1
    move 2 from 4 to 6
    move 2 from 2 to 1
    move 1 from 1 to 9
    move 2 from 8 to 5
    move 2 from 2 to 8
    move 2 from 5 to 2
    move 2 from 7 to 2
    move 1 from 3 to 1
    move 4 from 1 to 8
    move 1 from 9 to 5
    move 1 from 1 to 7
    move 1 from 2 to 8
    move 29 from 8 to 3
    move 15 from 3 to 2
    move 12 from 2 to 5
    move 1 from 1 to 6
    move 3 from 2 to 1
    move 6 from 3 to 8
    move 2 from 3 to 9
    move 1 from 6 to 7
    move 12 from 5 to 8
    move 2 from 7 to 1
    move 2 from 1 to 4
    move 2 from 4 to 2
    move 1 from 5 to 8
    move 1 from 3 to 6
    move 2 from 3 to 4
    move 3 from 1 to 4
    move 5 from 8 to 9
    move 4 from 4 to 2
    move 5 from 9 to 6
    move 26 from 6 to 8
    move 7 from 2 to 8
    move 3 from 3 to 1
    move 1 from 6 to 4
    move 14 from 8 to 6
    move 2 from 1 to 2
    move 1 from 1 to 3
    move 18 from 8 to 5
    move 15 from 8 to 2
    move 5 from 6 to 8
    move 4 from 5 to 8
    move 7 from 2 to 5
    move 2 from 9 to 6
    move 1 from 2 to 1
    move 7 from 2 to 3
    move 7 from 8 to 1
    move 2 from 6 to 3
    move 1 from 4 to 6
    move 2 from 8 to 6
    move 10 from 3 to 9
    move 18 from 5 to 8
    move 1 from 4 to 6
    move 2 from 1 to 9
    move 12 from 6 to 9
    move 1 from 6 to 9
    move 9 from 8 to 4
    move 6 from 1 to 2
    move 3 from 8 to 9
    move 14 from 9 to 8
    move 5 from 4 to 9
    move 2 from 4 to 5
    move 16 from 8 to 5
    move 12 from 5 to 4
    move 7 from 5 to 1
    move 1 from 1 to 8
    move 1 from 5 to 8
    move 1 from 4 to 9
    move 8 from 2 to 7
    move 12 from 4 to 3
    move 2 from 2 to 5
    move 1 from 3 to 2
    move 3 from 5 to 4
    move 1 from 4 to 8
    move 3 from 4 to 9
    move 18 from 9 to 8
    move 8 from 3 to 1
    move 5 from 8 to 1
    move 1 from 2 to 5
    move 3 from 7 to 1
    move 3 from 7 to 5
    move 1 from 8 to 9
    move 5 from 9 to 7
    move 2 from 3 to 6
    move 16 from 1 to 4
    move 14 from 8 to 6
    move 2 from 5 to 6
    move 4 from 1 to 6
    move 3 from 4 to 9
    move 15 from 6 to 1
    move 5 from 4 to 3
    move 2 from 8 to 2
    move 6 from 4 to 3
    move 15 from 1 to 5
    move 14 from 5 to 3
    move 5 from 6 to 2
    move 2 from 4 to 7
    move 1 from 1 to 6
    move 2 from 3 to 4
    move 3 from 8 to 1
    move 1 from 5 to 1
    move 5 from 7 to 1
    move 7 from 1 to 3
    move 3 from 6 to 2
    move 4 from 9 to 5
    move 2 from 4 to 3
    move 4 from 7 to 9
    move 8 from 2 to 9
    move 1 from 9 to 1
    move 2 from 2 to 8
    move 11 from 9 to 1
    move 6 from 5 to 1
    move 21 from 3 to 2
    move 1 from 8 to 5
    move 5 from 1 to 7
    move 12 from 1 to 8
    move 1 from 5 to 2
    move 5 from 3 to 2
    move 4 from 7 to 2
    move 1 from 7 to 8
    move 13 from 2 to 5
    move 13 from 2 to 5
    move 2 from 2 to 1
    move 1 from 1 to 9
    move 26 from 5 to 4
    move 3 from 2 to 7
    move 2 from 3 to 9
    move 1 from 1 to 6
    move 5 from 3 to 2
    move 2 from 9 to 6
    move 1 from 1 to 8
    move 3 from 1 to 6
    move 24 from 4 to 9
    move 13 from 9 to 1
    move 2 from 6 to 2
    move 3 from 7 to 5
    move 2 from 9 to 7
    move 8 from 8 to 3
    move 4 from 8 to 5
    move 2 from 7 to 2
    move 8 from 9 to 4
    move 10 from 1 to 2
    move 1 from 9 to 1
    move 1 from 9 to 2
    move 4 from 3 to 2
    move 4 from 1 to 8
    move 3 from 4 to 8
    move 12 from 2 to 3
    move 3 from 4 to 6
    move 5 from 3 to 2
    move 9 from 3 to 9
    move 4 from 2 to 9
    move 1 from 3 to 7
    move 6 from 8 to 2
    move 4 from 6 to 8
    move 1 from 3 to 8
    move 6 from 9 to 1
    move 2 from 1 to 8
    move 5 from 5 to 8
    move 3 from 6 to 8
    move 1 from 5 to 1
    move 7 from 8 to 2
    move 1 from 1 to 4
    move 1 from 4 to 6
    move 1 from 9 to 4
    move 1 from 5 to 9
    move 1 from 4 to 7
    move 12 from 8 to 2
    move 4 from 4 to 3
    move 2 from 3 to 1
    move 1 from 7 to 2
    move 1 from 6 to 8
    move 1 from 8 to 6
    move 4 from 9 to 3
    move 1 from 9 to 3
    move 13 from 2 to 3
    move 3 from 1 to 7
    move 2 from 9 to 4
    move 2 from 1 to 9
    move 2 from 7 to 2
    move 1 from 4 to 1
    move 2 from 7 to 5
    move 14 from 3 to 8
    move 1 from 8 to 5
    move 2 from 1 to 4
    move 2 from 3 to 4
    move 2 from 3 to 4
    move 10 from 8 to 3
    move 2 from 4 to 8
    move 1 from 9 to 3
    move 3 from 2 to 3
    move 16 from 2 to 4
    move 1 from 8 to 5
    move 11 from 3 to 4
    move 2 from 3 to 7
    move 3 from 5 to 1
    move 1 from 1 to 2
    move 3 from 2 to 5
    move 1 from 1 to 9
    move 2 from 7 to 4
    move 8 from 4 to 3
    move 1 from 6 to 7
    move 1 from 8 to 6
    move 1 from 5 to 1
    move 6 from 3 to 5
    move 2 from 1 to 3
    move 5 from 5 to 7
    move 2 from 7 to 2
    move 2 from 3 to 4
    move 4 from 7 to 1
    move 1 from 6 to 8
    move 1 from 2 to 1
    move 3 from 1 to 6
    move 2 from 9 to 6
    move 8 from 2 to 1
    move 2 from 6 to 2
    move 2 from 6 to 3
    move 6 from 3 to 5
    move 2 from 4 to 6
    move 2 from 2 to 9
    move 1 from 8 to 6
    move 2 from 6 to 5
    move 1 from 9 to 1
    move 11 from 5 to 8
    move 7 from 8 to 6
    move 23 from 4 to 1
    move 1 from 5 to 9
    move 1 from 4 to 6
    move 2 from 4 to 8
    move 1 from 3 to 1
    move 6 from 8 to 3
    move 2 from 9 to 6
    move 3 from 6 to 1
    move 3 from 8 to 7
    move 1 from 3 to 6
    move 18 from 1 to 2
    move 5 from 3 to 8
    move 13 from 2 to 9
    move 5 from 9 to 7
    move 1 from 8 to 6
    move 5 from 2 to 6
    move 2 from 1 to 7
    move 9 from 7 to 8
    move 11 from 8 to 6
    move 2 from 9 to 4
    move 16 from 6 to 1
    move 2 from 4 to 6
    move 1 from 8 to 9
    move 1 from 7 to 6
    move 8 from 1 to 5
    move 3 from 6 to 5
    move 8 from 6 to 4
    move 7 from 9 to 5
    move 1 from 8 to 1
    move 6 from 5 to 1
    move 9 from 5 to 7
    move 4 from 7 to 9
    move 1 from 4 to 8
    move 1 from 8 to 3
    move 1 from 1 to 8
    move 1 from 8 to 7
    move 22 from 1 to 3
    move 1 from 6 to 7
    move 2 from 9 to 4
    move 1 from 9 to 6
    move 1 from 9 to 4
    move 10 from 4 to 3
    move 1 from 1 to 2
    move 2 from 5 to 4
    move 27 from 3 to 8
    move 5 from 3 to 9";

    let max_len = data.lines().map(|l| l.len()).max().unwrap();
    let mut max_cols = 0;
    let mut piles: HashMap<usize, Vec<char>> = HashMap::new();
    for line in data.lines().rev() {
        println!("input line: {:?}", line);
        let mut colnum = 0;
        while colnum < max_len / 3 {
            // println!("colnum: {colnum}");
            if let Some(val) = line.get((1 + colnum * 4)..=(1 + colnum * 4)) {

                if val.trim() != "" {
                    println!("col{}: {val}",colnum+1);
                    piles
                        .entry(colnum + 1)
                        .and_modify(|e| e.push(val.chars().next().unwrap()))
                        .or_insert_with(|| vec![val.chars().next().unwrap()]);
                }
            }
            if colnum > max_cols {
                max_cols = colnum-1;
            }
            colnum += 1
        }
    }

    println!("{piles:?}");

    let move_parser = Regex::new(r"move (?P<num>\d+) from (?P<src>\d+) to (?P<dest>\d+)").unwrap();

    println!("Max_cols: {max_cols}");
    for line in move_input.lines() {
        println!("{}", line.trim());
        let moves = move_parser.captures(line).unwrap();
        let num = moves
            .name("num")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let src = moves
            .name("src")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let dest = moves
            .name("dest")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        println!("before move: {piles:?}");
        move_items(&mut piles, num, src, dest);
        println!("after move: {piles:?}");
    }
    for i in 1..max_cols+1 {
        print!("{}", piles.get(&i).unwrap().last().unwrap());
    }
    // answer is VCTFTJQCG
}
