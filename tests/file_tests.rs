use kafka_rust_cli::file::DirectoryPoller;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// integration tests for DirectoryPoller
/// these tests are external to the src directory
/// and test the public API of the module
#[test]
fn poll_directory_returns_error_for_missing_dir() {
    let temp_dir = tempfile::tempdir().expect("create temp dir failed");
    let missing_path = temp_dir.path().join("missing");

    let poller = DirectoryPoller::builder()
        .poll_interval_millis(0)
        .max_poll_cycles(1)
        .build();

    let result = poller.poll_directory(missing_path.to_string_lossy().as_ref());
    assert!(result.is_err());
}

#[test]
fn poll_directory_deletes_files_enabled() {
    let (temp_dir, file_path) = create_temp_dir_with_file();

    let poller = DirectoryPoller::builder()
        .delete_files(true)
        .poll_interval_millis(0)
        .max_poll_cycles(1)
        .build();

    let result = poller.poll_directory(temp_dir_to_string(&temp_dir).as_str());
    assert!(result.is_ok());
    assert!(!file_path.exists());
}

#[test]
fn poll_directory_keeps_files_disabled() {
    let (temp_dir, file_path) = create_temp_dir_with_file();

    let poller = DirectoryPoller::builder()
        .delete_files(false)
        .poll_interval_millis(0)
        .max_poll_cycles(1)
        .build();

    let result = poller.poll_directory(temp_dir_to_string(&temp_dir).as_str());
    assert!(result.is_ok());
    assert!(file_path.exists());
}

#[test]
fn poll_directory_runs_once() {
    use std::cell::Cell;
    use std::rc::Rc;

    let (temp_dir, file_path) = create_temp_dir_with_file();

    // Counter to track how many times closure is called
    // Cell provides simpler interior mutability for Copy types
    // Rc allows us to share ownership of the counter in the closure and the test function
    let call_count = Rc::new(Cell::new(0));
    let call_count_clone = Rc::clone(&call_count);

    // We will NOT delete files
    let poller = DirectoryPoller::builder()
        .delete_files(false)
        .poll_interval_millis(0)
        .keep_running(false)
        .on_file_content(move |_content| {
            call_count_clone.set(call_count_clone.get() + 1);
            Ok(())
        })
        .build();

    let result = poller.poll_directory(temp_dir_to_string(&temp_dir).as_str());
    assert!(result.is_ok());
    assert!(file_path.exists());

    assert_eq!(call_count.get(), 1);
}

#[test]
fn poll_directory_runs_multiple_cycles() {
    use std::cell::Cell;
    use std::rc::Rc;

    let poll_cycles = 3;
    let (temp_dir, file_path) = create_temp_dir_with_file();

    // Counter to track how many times closure is called
    let call_count = Rc::new(Cell::new(0));
    let call_count_clone = Rc::clone(&call_count);

    // We will NOT delete files
    // With 3 poll cycles the closure should
    // be called 3 times for the same file
    let poller = DirectoryPoller::builder()
        .delete_files(false)
        .poll_interval_millis(0)
        .max_poll_cycles(poll_cycles)
        .on_file_content(move |_content| {
            call_count_clone.set(call_count_clone.get() + 1);
            Ok(())
        })
        .build();

    let result = poller.poll_directory(temp_dir_to_string(&temp_dir).as_str());
    assert!(result.is_ok());
    assert!(file_path.exists());

    // Closure should have been called once per poll cycle
    assert_eq!(call_count.get(), poll_cycles);
}

#[test]
#[should_panic(expected = "Simulated error in callback")]
fn poll_directory_with_error_in_callback() {
    let (temp_dir, _file_path) = create_temp_dir_with_file();

    let poller = DirectoryPoller::builder()
        .delete_files(true)
        .poll_interval_millis(0)
        .max_poll_cycles(1)
        .on_file_content(move |_content| Err("Simulated error in callback".into()))
        .build();

    let _ = poller.poll_directory(temp_dir_to_string(&temp_dir).as_str());
}

fn create_temp_dir_with_file() -> (TempDir, PathBuf) {
    let temp_dir = tempfile::tempdir().expect("create temp dir failed");
    let file_path = temp_dir.path().join("sample.txt");
    fs::write(&file_path, "hello").expect("writing temp file failed");

    (temp_dir, file_path)
}

fn temp_dir_to_string(temp_dir: &TempDir) -> String {
    temp_dir.path().to_string_lossy().to_string()
}
