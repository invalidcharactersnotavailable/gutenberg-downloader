# Rust Downloader

This project is a high-performance, multithreaded file downloader written in Rust. It allows you to download multiple files concurrently from a specified URL base, with configurable options for the number of threads, target folder, and number of files to download.

## Features
- **Multithreaded Execution**: Uses Tokio's multithreaded runtime for efficient parallel downloads.
- **Configurable Options**: Specify the target folder, number of files, and number of threads via command-line arguments.
- **Performance Metrics**: Displays the total time taken, number of successful downloads, and number of failed downloads.

## Usage

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).

### Build and Run
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the program:
   ```bash
   cargo run -- --folder <target_folder> --files <number_of_files> --threads <number_of_threads>
   ```

### Example
To download 100 files into the `data` folder using 8 threads:
```bash
cargo run -- --folder data --files 100 --threads 8
```

## Output
The program will log the status of each file download and display a summary at the end:
```
File 1 downloaded successfully to data/pg1.txt
File 2 downloaded successfully to data/pg2.txt
...
Downloaded 100 files in 12.34 seconds using 8 threads (95 succeeded, 5 failed)
```

## Project Structure
- `src/main.rs`: The main program logic, including multithreaded file downloading and error handling.

## Dependencies
- [Tokio](https://tokio.rs/): For asynchronous runtime and multithreading.
- [Reqwest](https://docs.rs/reqwest/): For HTTP requests.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.