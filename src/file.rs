use std::error::Error;
/// File reading and directory polling
/// Reference: DefaultDirectoryPollingService
/// Reference: DefaultKafkaProducerUtil
use std::fs;
use std::path::Path;

// Type alias for file content callback
// Need to wrap closure in Box to allocate on the heap
// and use dynamic dispatch since we don't know the closure at compile time
// allowing us to use any closure that matches the signature
type FileContentCallback = Box<dyn Fn(&str) -> Result<(), Box<dyn Error>>>;

pub struct DirectoryPoller {
    keep_running: bool,
    delete_files: bool,
    poll_interval_millis: u64,
    max_poll_cycles: i32,
    on_file_content: FileContentCallback,
}

impl DirectoryPoller {
    pub fn builder() -> DirectoryPollerBuilder {
        DirectoryPollerBuilder::new()
    }

    // Poll directory for files
    pub fn poll_directory(&self, directory: &str) -> Result<(), Box<dyn Error>> {
        let directory_path = Path::new(directory);

        // Validate directory exists and is a directory
        // and fail fast if it is not valid
        // to avoid repeated attempts in next poll cycle
        match self.verify_directory(directory_path) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // Poll the directory
        log::info!("Polling directory: {}", self.file_name(directory_path));

        let mut poll_cycles = 0;
        let mut keep_running = true;

        while keep_running {
            let mut file_count = 0;
            for directory_iter in fs::read_dir(directory_path)? {
                let directory_entry = directory_iter?;
                let file_path = directory_entry.path();
                // Only process regular files
                // we will skip subdirectories, symlinks etc
                if file_path.is_file() {
                    file_count += 1;
                    self.process_file(&file_path);
                }
            }

            // end of poll cycle
            poll_cycles += 1;
            if file_count == 0 {
                log::info!("No files found on this poll cycle");
            }

            keep_running = self.should_continue_polling(poll_cycles);
        }

        Ok(())
    }

    fn verify_directory(&self, directory_path: &Path) -> Result<(), Box<dyn Error>> {
        if !directory_path.exists() {
            return Err(format!(
                "Directory does not exist: {}",
                self.file_name(directory_path)
            )
            .into());
        }
        if !directory_path.is_dir() {
            return Err(format!(
                "Path is not a directory: {}",
                self.file_name(directory_path)
            )
            .into());
        }
        Ok(())
    }

    // This will read the file content
    // then pass that to a closure that can be used to process the content
    // TODO: should we pass along Result using chaining instead of panicking in this function
    fn process_file(&self, file_path: &Path) {
        //-> Result<(), Box<dyn Error>> {
        log::info!("Processing file: {:?}", self.file_name(file_path));

        // options to handle Result
        // 1) match or if let (can deal w/ error now)
        // 2) unwrap/expect (panic on error)
        // 3) try operator (?) (propagate error up)

        // panic version of reading a file
        let content = std::fs::read_to_string(file_path)
            .unwrap_or_else(|_| panic!("Failed to read file {:?}", self.file_name(file_path)));

        // panic version of processing file content
        (self.on_file_content)(content.as_str()).unwrap_or_else(|_| {
            panic!(
                "Error processing content of file {:?}",
                self.file_name(file_path)
            )
        });

        self.delete_file(file_path);

        // match std::fs::read_to_string(file_path) {
        //     Ok(content) => {
        //         // process file content
        //         // TODO: accept a lambda/function to process content
        //         log::info!("File content: {}", content);

        //         // the file content is a String
        //         // and it will go out of scope at the end of this block
        //         // TODO: we probably should "move" ownership to our content handler
        //         // but maybe NOT since we will not need the content to outlive this block
        //         // TODO: our content handler will need to return Result
        //         // and then we need to decide how to handle failures in content processing

        //         // manage deletion of file
        //         self.delete_file(file_path);
        //     }
        //     Err(e) => {
        //         log::error!("Failed to read file {}: {}", self.file_name(file_path), e);
        //     }
        //}
        //Ok(())
    }

    // delete file if the delete_files flag is enabled
    // if deletion fails, log the error
    // but do not return an error from this function
    fn delete_file(&self, file_path: &Path) {
        if self.delete_files {
            // delete file logic
            if let Err(e) = std::fs::remove_file(file_path) {
                // TODO: this can result in processing the file
                // multiple times across poll cycles
                log::error!("Failed to delete file {}: {}", self.file_name(file_path), e);
            }
        } else {
            log::info!(
                "File deletion is disabled, skipping deletion for file: {}",
                self.file_name(file_path)
            );
        }
    }

    fn should_continue_polling(&self, poll_cycles: i32) -> bool {
        // max poll cycles takes precedence over the keep_running flag
        let continue_polling = if self.max_poll_cycles <= 0 {
            // max poll cycles is not enabled
            // use the keep_running flag
            self.keep_running
        } else {
            // max poll cycles is enabled
            // only continue if we have not reached the max cycles
            poll_cycles < self.max_poll_cycles
        };

        // if we are going to keep running, sleep for the configured delay
        if continue_polling {
            std::thread::sleep(std::time::Duration::from_millis(self.poll_interval_millis));
        }

        continue_polling
    }

    fn file_name(&self, path: &Path) -> String {
        let file_name = path.file_name().unwrap_or_default();
        file_name.to_string_lossy().to_string()
    }
}

/// Builder for DirectoryPoller
pub struct DirectoryPollerBuilder {
    keep_running: bool,
    delete_files: bool,
    poll_interval_millis: u64,
    max_poll_cycles: i32,
    // default no-op closure
    on_file_content: FileContentCallback,
}

impl DirectoryPollerBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DirectoryPollerBuilder {
            keep_running: false,
            delete_files: false,
            poll_interval_millis: 1000,
            max_poll_cycles: -1,
            on_file_content: Box::new(|_content| Ok(())),
        }
    }

    pub fn keep_running(mut self, keep_running: bool) -> Self {
        self.keep_running = keep_running;
        self
    }

    pub fn delete_files(mut self, delete_files: bool) -> Self {
        self.delete_files = delete_files;
        self
    }

    pub fn poll_interval_millis(mut self, poll_interval_millis: u64) -> Self {
        self.poll_interval_millis = poll_interval_millis;
        self
    }

    pub fn max_poll_cycles(mut self, max_poll_cycles: i32) -> Self {
        self.max_poll_cycles = max_poll_cycles;
        self
    }

    // wrap the callback function in a Box to allow for dynamic dispatch
    // and use static lifetime since we want the closure
    // to be valid for the life of the DirectoryPoller
    #[allow(dead_code)]
    pub fn on_file_content<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) -> Result<(), Box<dyn Error>> + 'static,
    {
        self.on_file_content = Box::new(callback);
        self
    }

    pub fn build(self) -> DirectoryPoller {
        DirectoryPoller {
            keep_running: self.keep_running,
            delete_files: self.delete_files,
            poll_interval_millis: self.poll_interval_millis,
            max_poll_cycles: self.max_poll_cycles,
            on_file_content: self.on_file_content,
        }
    }
}

/// unit tests for DirectoryPoller
/// these tests are internal to the src directory
/// and can test private functions and implementation details of the module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_directory_fails() {
        let poller = DirectoryPoller::builder().build();
        let result = poller.verify_directory(Path::new("nonexistent_dir"));
        assert!(result.is_err());
    }

    #[test]
    fn verify_directory_ok() {
        let temp_dir = tempfile::tempdir().expect("create temp dir failed");
        let poller = DirectoryPoller::builder().build();
        let result = poller.verify_directory(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn call_on_file_content() {
        let temp_dir = tempfile::tempdir().expect("create temp dir failed");
        let file_path = temp_dir.path().join("sample.txt");
        fs::write(&file_path, "test content").expect("writing temp file failed");

        let poller = DirectoryPoller::builder()
            .keep_running(false)
            .delete_files(false)
            .on_file_content(|content| {
                assert_eq!(content, "test content");
                Ok(())
            })
            .build();
        let result = poller.poll_directory(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    #[test]
    fn call_on_file_content_multi() {
        use std::cell::Cell;
        use std::rc::Rc;

        let temp_dir = tempfile::tempdir().expect("create temp dir failed");
        let file_path = temp_dir.path().join("sample.txt");
        fs::write(&file_path, "test content").expect("writing temp file failed");

        let poll_cycles = 3;

        // Use Cell for simpler interior mutability
        // and clone the Rc to share ownership between the test function and the closure
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = Rc::clone(&call_count);

        let poller = DirectoryPoller::builder()
            .max_poll_cycles(poll_cycles)
            .delete_files(false)
            .on_file_content(move |_content| {
                call_count_clone.set(call_count_clone.get() + 1);
                Ok(())
            })
            .build();

        let result = poller.poll_directory(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());
        assert!(file_path.exists());
        assert_eq!(call_count.get(), poll_cycles);
    }

    #[test]
    //#[should_panic(expected = "Simulated error in callback")]
    #[should_panic(expected = "Error processing content of file \"test_unit.txt\"")]
    fn call_on_file_content_error() {
        let temp_dir = tempfile::tempdir().expect("create temp dir failed");
        let file_path = temp_dir.path().join("test_unit.txt");
        fs::write(&file_path, "test content").expect("writing temp file failed");

        let poller = DirectoryPoller::builder()
            .keep_running(false)
            .delete_files(false)
            .on_file_content(|content| {
                assert_eq!(content, "test content");
                Err("Simulated error in callback".into())
            })
            .build();
        let _ = poller.poll_directory(temp_dir.path().to_str().unwrap());
        // TODO: we should be able to propagate the error instead of panicking in the callback
        // assert!(result.is_err());
        // assert!(file_path.exists());
    }
}
