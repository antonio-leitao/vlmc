// use hashbrown::{HashMap, HashSet};
// use rand::Rng;
// use std::thread;
// use std::time::Instant;

// #[derive(Debug, PartialEq)]
// struct Node {
//     count: usize,
//     precedents: HashSet<u32>,
// }
// impl Node {
//     fn new() -> Self {
//         Node {
//             count: 0,
//             precedents: HashSet::new(),
//         }
//     }
// }

// fn count_subsequences_parallel(data: Vec<Vec<u32>>, max_depth: usize) -> HashMap<Vec<u32>, Node> {
//     let num_threads = num_cpus::get(); // get the number of available CPU cores
//     let chunk_size = data.len() / num_threads; // divide the data into equal-sized chunks
//     let mut threads = Vec::with_capacity(num_threads);

//     // spawn threads to process each chunk
//     for chunk in data.chunks(chunk_size) {
//         let chunk_data = chunk.to_vec(); // convert the chunk slice to a vector

//         let handle = thread::spawn(move || count_subsequences(chunk_data, max_depth));

//         threads.push(handle);
//     }

//     // collect the results from all threads and combine them into a single HashMap
//     let mut counts = HashMap::new();

//     for handle in threads {
//         let chunk_counts = handle.join().unwrap();
//         for (k, v) in chunk_counts {
//             let node = counts.entry(k).or_insert(Node::new());
//             node.count += v.count;
//             node.precedents.extend(v.precedents);
//         }
//     }

//     counts
// }

// fn count_subsequences(data: Vec<Vec<u32>>, max_depth: usize) -> HashMap<Vec<u32>, Node> {
//     let mut counts: HashMap<Vec<u32>, Node> = HashMap::new();
//     for seq in data {
//         let n = seq.len().min(max_depth);

//         for i in 0..seq.len() {
//             for j in i..(i + n).min(seq.len()) {
//                 let subseq = &seq[i..=j];
//                 let node = counts.entry(subseq.to_vec()).or_insert(Node::new());
//                 node.count += 1;

//                 if i > 0 {
//                     node.precedents.insert(seq[i - 1]);
//                 }
//             }
//         }
//     }
//     counts
// }

// fn main() {
//     let mut rng = rand::thread_rng();
//     let data: Vec<Vec<u32>> = (0..10000)
//         .map(|_| (0..100).map(|_| rng.gen_range(0..10)).collect())
//         .collect();
//     let max_depth:usize = 10;

//     let start_time = Instant::now();
//     let counts = count_subsequences(data.clone(), max_depth);
//     let end_time = Instant::now();
//     println!(
//         "Sequential execution time: {} ms",
//         (end_time - start_time).as_millis()
//     );

//     let start_time = Instant::now();
//     let counts_parallel = count_subsequences_parallel(data.clone(), max_depth);
//     let end_time = Instant::now();
//     println!(
//         "Parallel execution time: {} ms",
//         (end_time - start_time).as_millis()
//     );

//     assert_eq!(counts, counts_parallel);

//     let sequential_sum: usize = counts.into_values().map(|node| node.count).sum();
//     let parallel_sum: usize = counts_parallel.into_values().map(|node| node.count).sum();

//     assert_eq!(sequential_sum, parallel_sum);
// }
