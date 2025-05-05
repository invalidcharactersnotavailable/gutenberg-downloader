use std::env;
use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest;

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let folder = args.iter().position(|arg| arg == "--folder")
        .and_then(|i| args.get(i + 1))
        .expect("Usage: --folder <folder_path> --files <number_of_files>");
    let files: usize = args.iter().position(|arg| arg == "--files")
        .and_then(|i| args.get(i + 1))
        .expect("Usage: --folder <folder_path> --files <number_of_files>")
        .parse()
        .expect("Invalid number for --files");

    // Ensure the folder exists
    if !Path::new(folder).exists() {
        fs::create_dir_all(folder).expect("Failed to create folder");
    }

    // Base URL for downloading files
    let base_url = "https://gutenberg.org/cache/epub";

    // Download files asynchronously
    let mut tasks = vec![];
    for i in 1..=files {
        let url = format!("{}/{}/pg{}.txt", base_url, i, i);
        let file_path = format!("{}/pg{}.txt", folder, i);

        tasks.push(tokio::spawn(async move {
            match download_file(&url, &file_path).await {
                Ok(_) => println!("File {} downloaded successfully to {}", i, file_path),
                Err(e) => eprintln!("Failed to download file {}: {}", i, e),
            }
        }));
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.expect("Task panicked");
    }
}

async fn download_file(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Send a GET request to the URL
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        // Write the response body to the file asynchronously
        let mut file = File::create(file_path).await?;
        let content = response.bytes().await?;
        file.write_all(&content).await?;
        Ok(())
    } else {
        Err(format!("Failed to download file: HTTP {}", response.status()).into())
    }
}