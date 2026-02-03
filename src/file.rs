/// File reading and directory polling
/// Reference: DefaultDirectoryPollingService
/// Reference: DefaultKafkaProducerUtil
use std::fs;
use std::path::Path;

pub struct DirectoryPoller;

impl DirectoryPoller {
    /// Poll directory for files
    pub fn poll_directory(&self, directory: &str) -> Result<(), Box<dyn std::error::Error>> {
        let directory_path = Path::new(directory);

        // Validate directory exists and is a directory
        // and fail fast if it is not valid
        // to avoid repeated attempts in next poll cycle
        if !directory_path.exists() {
            return Err(format!("Directory does not exist: {}", directory).into());
        }
        if !directory_path.is_dir() {
            return Err(format!("Path is not a directory: {}", directory).into());
        }

        log::info!("Polling directory: {}", self.file_name(directory_path));

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
                    log::info!("Found file: {:?}", self.file_name(&file_path));
                }
            }

            // end of poll cycle
            if file_count == 0 {
                log::info!("No files found in directory on that poll cycle");
            } else {
                log::info!("Processed {} files on that poll cycle", file_count);
            }

            keep_running = self.should_continue_polling();
        }

        Ok(())
    }

    fn should_continue_polling(&self) -> bool {
        // For now we just poll once
        false
    }

    fn file_name(&self, path: &Path) -> String {
        let file_name = path.file_name().unwrap_or_default();
        file_name.to_string_lossy().to_string()
    }

}
