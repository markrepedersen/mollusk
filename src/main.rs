/// A distributed storage system written in rust.
/// ********
/// Robustness techniques:
/// - Sharding: Data may not fit all on one machine; sharding splits data onto multiple different machines.
/// - Replication: To make the system fault tolerant, replicate data in case any one machine goes down.
/// - Consistency: Data must be consistent between replicas/machines.
/// ********
/// Optimization techniques:
/// - Read throughput: There may be lots and lots of requests, therefore, relevant data should be in main memory as much as possible.
/// - Creating a cache system: See above for motivation.
fn main() {
    println!("Hello, world!");
}
