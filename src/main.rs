use hashbrown::{HashMap, HashSet};
use std::thread;
use rand::Rng;
use std::time::Instant;

fn count_subsequences_parallel(data: Vec<Vec<u32>>, max_depth: usize) -> HashMap<Vec<u32>, (usize, HashSet<u32>)> {
    let num_threads = num_cpus::get(); // get the number of available CPU cores
    let chunk_size = data.len() / num_threads; // divide the data into equal-sized chunks
    let mut threads = Vec::with_capacity(num_threads);

    // spawn threads to process each chunk
    for chunk in data.chunks(chunk_size) {
        let chunk_data = chunk.to_vec(); // convert the chunk slice to a vector

        let handle = thread::spawn(move || {
            count_subsequences(chunk_data, max_depth)
        });

        threads.push(handle);
    }

    // collect the results from all threads and combine them into a single HashMap
    let mut counts = HashMap::new();

    for handle in threads {
        let chunk_counts = handle.join().unwrap();
        for (k, v) in chunk_counts {
            let count = counts.entry(k).or_insert((0, HashSet::new()));
            count.0 += v.0;
            count.1.extend(v.1);
        }
    }

    counts
}

struct Node {
    count: usize,
    precedents: HashSet<u32>,
}

fn count_subsequences(data:Vec<Vec<u32>>,max_depth:usize)->HashMap<Vec<u32>,(usize,HashSet<u32>)>{
    let mut counts: HashMap<Vec<u32>, (usize, HashSet<u32>)> = HashMap::new();
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
    counts
}

// fn main() {
//     let data = vec![
//         vec![1, 2, 3, 4],
//         vec![2, 3, 4, 5],
//         vec![1, 2, 3, 5],
//         vec![1, 2],
//     ];
//     let max_depth = 3;
//    let counts = count_subsequences(data, max_depth);
//     for (subseq, (count, precedents)) in counts {
//         println!("Subsequence {:?} has count {} and precedents {:?}", subseq, count, precedents);
//     }
// }


fn main() {
    let mut rng = rand::thread_rng();
    let data: Vec<Vec<u32>> = (0..1000)
        .map(|_| (0..100).map(|_| rng.gen_range(0..10)).collect())
        .collect();

    let start_time = Instant::now();
    let counts = count_subsequences(data.clone(), 5);
    let end_time = Instant::now();
    println!(
        "Sequential execution time: {} ms",
        (end_time - start_time).as_millis()
    );

    let start_time = Instant::now();
    let counts_parallel = count_subsequences_parallel(data.clone(), 5);
    let end_time = Instant::now();
    println!(
        "Parallel execution time: {} ms",
        (end_time - start_time).as_millis()
    );
    assert_eq!(counts, counts_parallel);
    let sequential_sum:usize=counts.into_values().map(|(counts,_)|counts).sum();
    let parallel_sum:usize=counts_parallel.into_values().map(|(counts,_)|counts).sum();


    
    assert_eq!(sequential_sum, parallel_sum);

}
