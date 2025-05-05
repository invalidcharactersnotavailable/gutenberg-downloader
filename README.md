# gutenberg downloader
## Usage

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).

### Build and Run
1. Clone the repository:
   ```bash
   git clone https://github.com/invalidcharactersnotavailable/gutenberg-downloader.git
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
