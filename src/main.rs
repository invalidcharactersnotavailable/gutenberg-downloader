use std::env;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use reqwest::Client;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // Start the timer
    let start_time: Instant = Instant::now();

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let folder: &String = args.iter().position(|arg: &String| arg == "--folder")
        .and_then(|i: usize| args.get(i + 1))
        .expect("Usage: --folder <folder_path> --files <number_of_files> --threads <number_of_threads>");
    let files: usize = args.iter().position(|arg: &String| arg == "--files")
        .and_then(|i: usize| args.get(i + 1))
        .expect("Usage: --folder <folder_path> --files <number_of_files> --threads <number_of_threads>")
        .parse()
        .expect("Invalid number for --files");
    let threads: usize = args.iter().position(|arg: &String| arg == "--threads")
        .and_then(|i: usize| args.get(i + 1))
        .expect("Usage: --folder <folder_path> --files <number_of_files> --threads <number_of_threads>")
        .parse()
        .expect("Invalid number for --threads");

    // Ensure the folder exists
    if !Path::new(folder).exists() {
        fs::create_dir_all(folder).expect("Failed to create folder");
    }

    // Base URL for downloading files
    let base_url: &'static str = "https://gutenberg.org/cache/epub";

    // Atomic counters for success and failure
    let success_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let failure_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    // Create a shared HTTP client
    let client: Arc<Client> = Arc::new(Client::new());

    // Semaphore to limit concurrency
    let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(threads));

    // Download files asynchronously in parallel
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = vec![];
    for i in 1..=files {
        let url: String = format!("{}/{}/pg{}.txt", base_url, i, i);
        let file_path: String = format!("{}/pg{}.txt", folder, i);

        let client: Arc<Client> = Arc::clone(&client);
        let success_count: Arc<AtomicUsize> = Arc::clone(&success_count);
        let failure_count: Arc<AtomicUsize> = Arc::clone(&failure_count);
        let semaphore: Arc<Semaphore> = Arc::clone(&semaphore);

        tasks.push(tokio::spawn(async move {
            let _permit: tokio::sync::SemaphorePermit<'_> = semaphore.acquire().await.unwrap();
            match download_file(&client, &url, &file_path).await {
                Ok(_) => {
                    success_count.fetch_add(1, Ordering::SeqCst);
                    println!("File {} downloaded successfully to {}", i, file_path);
                }
                Err(e) => {
                    failure_count.fetch_add(1, Ordering::SeqCst);
                    eprintln!("Failed to download file {}: {}", i, e);
                }
            }
        }));
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.expect("Task panicked");
    }

    // Calculate and display elapsed time
    let elapsed_time: std::time::Duration = start_time.elapsed();
    println!(
        "Downloaded {} files in {:.2?} seconds using {} threads ({} succeeded, {} failed)",
        files,
        elapsed_time.as_secs_f64(),
        threads,
        success_count.load(Ordering::SeqCst),
        failure_count.load(Ordering::SeqCst)
    );
}

async fn download_file(client: &Client, url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Send a GET request to the URL
    let response: reqwest::Response = client.get(url).send().await?;
    if response.status().is_success() {
        // Write the response body to the file asynchronously
        let mut file: File = File::create(file_path).await?;
        let content = response.bytes().await?;
        file.write_all(&content).await?;
        Ok(())
    } else {
        Err(format!("Failed to download file: HTTP {}", response.status()).into())
    }
}