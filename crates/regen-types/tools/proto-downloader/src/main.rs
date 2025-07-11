use std::path::Path;

use anyhow::Result;
use flate2::read::GzDecoder;
use tempfile::tempdir;
use tar::Archive;
use reqwest;
use walkdir::WalkDir;
use std::fs;

use clap::Parser;



struct GithubRepo {
    owner: String,
    repo: String,
    tag: String,
}

impl GithubRepo {
    fn new(owner: &str, repo: &str, tag: &str) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            tag: tag.to_string(),
        }
    }

    fn download(&self, output_dir: &Path) -> Result<()> {
        let url = format!("https://github.com/{}/{}/archive/{}.tar.gz", self.owner, self.repo, self.tag);
        let response = reqwest::blocking::get(&url)?;
        let bytes = response.bytes()?;
        let tar = GzDecoder::new(bytes.as_ref());
        let mut archive = Archive::new(tar);
        archive.unpack(output_dir)?;
        Ok(())
    }
}

fn extract_protos(source_dir: &Path, dest_dir: &Path) -> Result<()> {

    // check if output directory exists
    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir)?;
    }

    for entry in WalkDir::new(source_dir) {
        let entry = entry?;
        let path = entry.path();
        
        // Only process .proto files
        if path.extension().map_or(false, |ext| ext == "proto") {
            // Get the relative path from source to this file
            let relative_path = path.strip_prefix(source_dir)?;
            
            // Create the destination path maintaining the structure
            let dest_file_path = dest_dir.join(relative_path);
            
            // Create parent directories if they don't exist
            if let Some(parent) = dest_file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Copy the file
            fs::copy(path, &dest_file_path)?;
            
            println!("Copied: {} -> {}", 
                path.display(), 
                dest_file_path.display()
            );
        }
    }
    Ok(())
}


#[derive(Parser, Debug)]
#[command(name = "proto-downloader")]
#[command(about = "A tool for downloading protobuf definitions from GitHub repositories")]
#[command(version)]
struct Cli {
    /// Output directory for downloaded proto files
    #[arg(short, long, default_value = "./protos")]
    output: String,

    /// GitHub repository owner (e.g., "regen-network")
    #[arg(short = 'o', long, default_value = "regen-network")]
    owner: String,

    /// GitHub repository name (e.g., "regen-ledger")
    #[arg(short = 'r', long, default_value = "regen-ledger")]
    repo: String,

    /// Git tag or version to download (e.g., "v6.0.0")
    #[arg(short, long, default_value = "v6.0.0")]
    tag: String,

    /// Proto directory to download from the repo
    #[arg(short, long, default_value = "proto")]
    proto_dir: String,
}


fn main() -> anyhow::Result<()> {

    let cli = Cli::parse();

    let repo = GithubRepo::new(&cli.owner, &cli.repo, &cli.tag);
    let output_dir = Path::new(&cli.output);
    let proto_dir = Path::new(&cli.proto_dir);


    let temp_dir = tempdir()?;
    repo.download(temp_dir.path())?;

    // GitHub archives extract to a directory named {repo}-{tag} but strip 'v' prefix from tags
    let tag_without_v = cli.tag.strip_prefix('v').unwrap_or(&cli.tag);
    let extracted_repo_dir = temp_dir.path().join(format!("{}-{}", cli.repo, tag_without_v));
    let proto_source_path = extracted_repo_dir.join(proto_dir);

    extract_protos(&proto_source_path, &output_dir)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_file(dir: &Path, filename: &str, content: &str) -> Result<()> {
        let file_path = dir.join(filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_extract_protos_basic_functionality() -> Result<()> {
        let temp_source = tempdir()?;
        let temp_dest = tempdir()?;
        
        // Create a basic proto file
        create_test_file(temp_source.path(), "test.proto", "syntax = \"proto3\";\n")?;
        
        extract_protos(temp_source.path(), temp_dest.path())?;
        
        // Verify the file was copied
        let dest_file = temp_dest.path().join("test.proto");
        assert!(dest_file.exists());
        
        // Verify content is preserved
        let content = fs::read_to_string(&dest_file)?;
        assert_eq!(content, "syntax = \"proto3\";\n");
        
        Ok(())
    }

    #[test]
    fn test_extract_protos_preserves_directory_structure() -> Result<()> {
        let temp_source = tempdir()?;
        let temp_dest = tempdir()?;
        
        // Create proto files in nested directories
        create_test_file(temp_source.path(), "root.proto", "// root proto\n")?;
        create_test_file(temp_source.path(), "subdir/nested.proto", "// nested proto\n")?;
        create_test_file(temp_source.path(), "subdir/deep/deeper.proto", "// deep proto\n")?;
        
        extract_protos(temp_source.path(), temp_dest.path())?;
        
        // Verify all files were copied with correct structure
        assert!(temp_dest.path().join("root.proto").exists());
        assert!(temp_dest.path().join("subdir/nested.proto").exists());
        assert!(temp_dest.path().join("subdir/deep/deeper.proto").exists());
        
        // Verify content is preserved
        let content = fs::read_to_string(temp_dest.path().join("subdir/deep/deeper.proto"))?;
        assert_eq!(content, "// deep proto\n");
        
        Ok(())
    }

    #[test]
    fn test_extract_protos_filters_non_proto_files() -> Result<()> {
        let temp_source = tempdir()?;
        let temp_dest = tempdir()?;
        
        // Create various file types
        create_test_file(temp_source.path(), "test.proto", "// proto file\n")?;
        create_test_file(temp_source.path(), "readme.md", "# README\n")?;
        create_test_file(temp_source.path(), "config.json", "{}\n")?;
        create_test_file(temp_source.path(), "script.sh", "#!/bin/bash\n")?;
        create_test_file(temp_source.path(), "noextension", "no extension\n")?;
        
        extract_protos(temp_source.path(), temp_dest.path())?;
        
        // Only .proto files should be copied
        assert!(temp_dest.path().join("test.proto").exists());
        assert!(!temp_dest.path().join("readme.md").exists());
        assert!(!temp_dest.path().join("config.json").exists());
        assert!(!temp_dest.path().join("script.sh").exists());
        assert!(!temp_dest.path().join("noextension").exists());
        
        Ok(())
    }

    #[test]
    fn test_extract_protos_handles_empty_source_directory() -> Result<()> {
        let temp_source = tempdir()?;
        let temp_dest = tempdir()?;
        
        // Don't create any files in source
        extract_protos(temp_source.path(), temp_dest.path())?;
        
        // Destination should exist but be empty
        assert!(temp_dest.path().exists());
        let entries: Vec<_> = WalkDir::new(temp_dest.path())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();
        assert_eq!(entries.len(), 0);
        
        Ok(())
    }

    #[test]
    fn test_extract_protos_creates_destination_directory() -> Result<()> {
        let temp_source = tempdir()?;
        let temp_dest = tempdir()?;
        
        // Create a nested destination path that doesn't exist
        let dest_path = temp_dest.path().join("new/nested/path");
        
        create_test_file(temp_source.path(), "test.proto", "// test proto\n")?;
        
        extract_protos(temp_source.path(), &dest_path)?;
        
        // Verify the destination directory was created
        assert!(dest_path.exists());
        assert!(dest_path.join("test.proto").exists());
        
        Ok(())
    }

    #[test]
    fn test_extract_protos_handles_mixed_case_extensions() -> Result<()> {
        let temp_source = tempdir()?;
        let temp_dest = tempdir()?;
        
        // Create files with different case extensions
        create_test_file(temp_source.path(), "test.proto", "// lowercase proto\n")?;
        create_test_file(temp_source.path(), "test_upper.PROTO", "// uppercase proto\n")?;
        create_test_file(temp_source.path(), "test_mixed.Proto", "// mixed case proto\n")?;
        
        extract_protos(temp_source.path(), temp_dest.path())?;
        
        // Only .proto files should be copied - the comparison is case-sensitive
        assert!(temp_dest.path().join("test.proto").exists());
        
        // Check what files were actually copied to understand the behavior
        let files_copied: Vec<_> = WalkDir::new(temp_dest.path())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();
        
        // The current implementation uses case-sensitive comparison with "proto"
        // so only files with exactly ".proto" extension should be copied
        assert_eq!(files_copied.len(), 1);
        assert!(files_copied[0].path().file_name().unwrap().to_str().unwrap().ends_with(".proto"));
        
        Ok(())
    }

    #[test]
    fn test_extract_protos_handles_complex_directory_structure() -> Result<()> {
        let temp_source = tempdir()?;
        let temp_dest = tempdir()?;
        
        // Create a complex directory structure with mixed files
        create_test_file(temp_source.path(), "api/v1/user.proto", "// user API\n")?;
        create_test_file(temp_source.path(), "api/v1/auth.proto", "// auth API\n")?;
        create_test_file(temp_source.path(), "api/v1/README.md", "# API v1\n")?;
        create_test_file(temp_source.path(), "api/v2/user.proto", "// user API v2\n")?;
        create_test_file(temp_source.path(), "common/types.proto", "// common types\n")?;
        create_test_file(temp_source.path(), "common/utils.go", "// Go utils\n")?;
        
        extract_protos(temp_source.path(), temp_dest.path())?;
        
        // Verify only .proto files were copied with correct structure
        assert!(temp_dest.path().join("api/v1/user.proto").exists());
        assert!(temp_dest.path().join("api/v1/auth.proto").exists());
        assert!(temp_dest.path().join("api/v2/user.proto").exists());
        assert!(temp_dest.path().join("common/types.proto").exists());
        
        // Verify non-proto files were not copied
        assert!(!temp_dest.path().join("api/v1/README.md").exists());
        assert!(!temp_dest.path().join("common/utils.go").exists());
        
        // Verify content is preserved
        let content = fs::read_to_string(temp_dest.path().join("api/v1/user.proto"))?;
        assert_eq!(content, "// user API\n");
        
        Ok(())
    }

    // Integration tests
    #[test]
    #[ignore] 
    fn test_integration_regen_network_v6_0_0() -> Result<()> {
        let temp_dest = tempdir()?;
        
        // Create GithubRepo instance for regen-network/regen-ledger v6.0.0
        let repo = GithubRepo::new("regen-network", "regen-ledger", "v6.0.0");
        
        // Download the repository
        let temp_download = tempdir()?;
        repo.download(temp_download.path())?;
        
        // Extract protos from the downloaded repo
        let proto_source_path = temp_download.path().join("regen-ledger-6.0.0/proto");
        extract_protos(&proto_source_path, temp_dest.path())?;
        
        // Verify that proto files were extracted
        let extracted_files: Vec<_> = WalkDir::new(temp_dest.path())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "proto"))
            .collect();
        
        // regen-ledger should have multiple proto files
        assert!(extracted_files.len() > 0, "Should have extracted at least one proto file");
        
        // Verify some expected proto files exist (these are known to exist in regen-ledger)
        let has_regen_proto = extracted_files.iter().any(|entry| {
            entry.path().to_str().unwrap().contains("regen") && 
            entry.path().extension().map_or(false, |ext| ext == "proto")
        });
        
        assert!(has_regen_proto, "Should have found regen-related proto files");
        
        // Verify that all extracted files have .proto extension
        for entry in &extracted_files {
            assert!(entry.path().extension().map_or(false, |ext| ext == "proto"));
        }
        
        println!("Successfully extracted {} proto files from regen-ledger v6.0.0", extracted_files.len());
        
        Ok(())
    }

    #[test]
    #[ignore] 
    fn test_integration_github_repo_download() -> Result<()> {
        let temp_dir = tempdir()?;
        
        // Test downloading from regen-network/regen-ledger
        let repo = GithubRepo::new("regen-network", "regen-ledger", "v6.0.0");
        repo.download(temp_dir.path())?;
        
        // Verify the download created the expected directory structure
        let extracted_dir = temp_dir.path().join("regen-ledger-6.0.0");
        assert!(extracted_dir.exists(), "Downloaded repository directory should exist");
        
        // Verify proto directory exists
        let proto_dir = extracted_dir.join("proto");
        assert!(proto_dir.exists(), "Proto directory should exist in the downloaded repo");
        
        // Verify there are some files in the proto directory
        let proto_files: Vec<_> = WalkDir::new(&proto_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();
        
        assert!(proto_files.len() > 0, "Proto directory should contain files");
        
        Ok(())
    }

    #[test]
    #[ignore] // This test requires network access
    fn test_integration_end_to_end() -> Result<()> {
        let temp_dest = tempdir()?;
        
        // Simulate the full CLI workflow
        let repo = GithubRepo::new("regen-network", "regen-ledger", "v6.0.0");
        let temp_download = tempdir()?;
        
        // Step 1: Download the repository
        repo.download(temp_download.path())?;
        
        // Step 2: Extract protos (this simulates the main function logic)
        let proto_source = temp_download.path().join("regen-ledger-6.0.0").join("proto");
        extract_protos(&proto_source, temp_dest.path())?;
        
        // Step 3: Verify the results
        let extracted_files: Vec<_> = WalkDir::new(temp_dest.path())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();
        
        // Verify we got some proto files
        assert!(extracted_files.len() > 0, "Should have extracted proto files");
        
        // Verify all extracted files are .proto files
        for entry in &extracted_files {
            assert!(entry.path().extension().map_or(false, |ext| ext == "proto"),
                   "All extracted files should be .proto files, found: {:?}", entry.path());
        }
        
        // Verify directory structure is preserved
        let has_nested_structure = extracted_files.iter().any(|entry| {
            entry.path().components().count() > 2 // More than just temp_dir/filename
        });
        
        assert!(has_nested_structure, "Should preserve nested directory structure");
        
        // Verify files have content
        for entry in extracted_files.iter().take(3) { // Check first 3 files
            let content = fs::read_to_string(entry.path())?;
            assert!(!content.is_empty(), "Proto files should not be empty");
            assert!(content.contains("syntax") || content.contains("package") || content.contains("message"),
                   "Proto files should contain protobuf syntax");
        }
        
        println!("End-to-end test successful! Extracted {} proto files", extracted_files.len());
        
        Ok(())
    }
}





