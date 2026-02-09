/// File reading and directory polling
/// Reference: DefaultDirectoryPollingService
/// Reference: DefaultKafkaProducerUtil
use std::fs;
use std::path::Path;

pub struct DirectoryPoller {
    keep_running: bool,
    delete_files: bool,
    poll_interval_millis: u64,
    max_poll_cycles: i32,
}

impl DirectoryPoller {
    pub fn builder() -> DirectoryPollerBuilder {
        DirectoryPollerBuilder::new()
    }

    // Poll directory for files
    pub fn poll_directory(&self, directory: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    fn verify_directory(&self, directory_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

    fn process_file(&self, file_path: &Path) {
        log::info!("Processing file: {:?}", self.file_name(file_path));

        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                // process file content 
                // TODO: accept a lambda/function to process content
                log::info!("File content: {}", content);

                // manage deletion of file
                self.delete_file(file_path);
            }
            Err(e) => {
                log::error!("Failed to read file {}: {}", self.file_name(file_path), e);
            }
        }
    }

    fn delete_file(&self, file_path: &Path) {
        if self.delete_files {
            // delete file logic
            if let Err(e) = std::fs::remove_file(file_path) {
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
        let continue_polling = if self.max_poll_cycles <=0 {
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
}

impl DirectoryPollerBuilder {
    pub fn new() -> Self {
        DirectoryPollerBuilder {
            keep_running: false,
            delete_files: false,
            poll_interval_millis: 1000,
            max_poll_cycles: -1,
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

    pub fn build(self) -> DirectoryPoller {
        DirectoryPoller {
            keep_running: self.keep_running,
            delete_files: self.delete_files,
            poll_interval_millis: self.poll_interval_millis,
            max_poll_cycles: self.max_poll_cycles,
        }
    }
}