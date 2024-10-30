pub struct ThreadPool {}

impl ThreadPool {
    /// Create a new LocalThreadPool with num_threads threads.
    ///
    /// Errors:
    /// - If num_threads is 0, return an error
    pub fn new(num_threads: usize) {
        todo!()
    }

    /// Execute the provided function on the thread pool
    ///
    /// Errors:
    /// - If we fail to send a message, report an error
    pub fn execute<F>(&self, f: F) {
        todo!()
    }
    /// Retrieve any results from the thread pool that have been computed
    pub fn get_results(&self) {
        todo!()
    }
}
