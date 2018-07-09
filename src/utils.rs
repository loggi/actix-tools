use num_cpus;

/// Computes the number of Sync workers based on the block factor.
/// More io time (block time) implicates in a larger number of workers.
pub fn n_workers(block_factor: f32) -> usize {
    if block_factor >= 1.0 {
        panic!("Block factor should be lower than 1.0");
    }

    let cpus = num_cpus::get() as f32;
    let multiplier = 1. / (1.0 - block_factor);
    (cpus * multiplier).ceil() as usize
}
