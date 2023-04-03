use hashbrown::{HashMap, HashSet};

struct Node {
    count: usize,
    children: HashSet<u32>,
}

fn main() {
    let data = vec![
        vec![1, 2, 3, 4],
        vec![2, 3, 4, 5],
        vec![1, 2, 3, 5],
        vec![1, 2],
    ];
    let mut counts: HashMap<Vec<u32>, (usize, HashSet<u32>)> = HashMap::new();
    let max_depth = 3;
    for seq in data {
        let n = seq.len().min(max_depth);

        for i in 0..seq.len() {
            for j in i..(i + n).min(seq.len()) {
                let subseq = &seq[i..=j];
                let count = counts.entry(subseq.to_vec()).or_insert((0, HashSet::new()));
                count.0 += 1;

                if i > 0 {
                    count.1.insert(seq[i-1]);
                   
                }
            }
        }
    }
    for (subseq, (count, precedents)) in counts {
        println!("Subsequence {:?} has count {} and precedents {:?}", subseq, count, precedents);
    }
}
