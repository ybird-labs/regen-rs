use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "proto-compiler")]
#[command(about = "Compile protobuf files to Rust code using prost and tonic")]
struct Args {
    /// Input directory containing proto files
    #[arg(short, long, default_value = ".")]
    input: PathBuf,
    
    /// Output directory for generated Rust files
    #[arg(short, long, default_value = "src/gen")]
    output: PathBuf,
    
    /// Enable gRPC client generation
    #[arg(long)]
    with_client: bool,
    
    /// Enable gRPC server generation
    #[arg(long)]
    with_server: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Find all proto files
    let proto_files = find_proto_files(&args.input)?;
    
    if proto_files.is_empty() {
        println!("No proto files found in {}", args.input.display());
        return Ok(());
    }
    
    println!("Found {} proto files:", proto_files.len());
    for proto_file in &proto_files {
        println!("  - {}", proto_file.display());
    }
    
    // Create output directory
    std::fs::create_dir_all(&args.output)?;
    
    // Configure prost-build
    let mut prost_build = prost_build::Config::new();
    prost_build.out_dir(&args.output);
    prost_build.compile_well_known_types();
    prost_build.enable_type_names();
    
    // Configure external type mappings (like ibc-proto-rs)
    prost_build.extern_path(".google.protobuf", "::tendermint_proto::google::protobuf");
    prost_build.extern_path(".tendermint", "::tendermint_proto");
    prost_build.extern_path(".cosmos", "::cosmos_sdk_proto::cosmos");
    prost_build.extern_path(".cosmos.ics23.v1", "::ics23");
    
    // Set up include paths to resolve dependencies
    let include_paths = vec![
        args.input.join("googleapis"),
        args.input.join("cosmos-sdk"),
        args.input.join("tendermint"),
        args.input.clone(),
        // Also add parent directory for cross-dependencies
        args.input.parent().unwrap_or(&args.input).join("googleapis"),
        args.input.parent().unwrap_or(&args.input).join("cosmos-sdk"),
        args.input.parent().unwrap_or(&args.input).join("tendermint"),
        args.input.parent().unwrap_or(&args.input).to_path_buf(),
    ];
    
    // Compile with prost for types
    println!("Compiling protobuf types...");
    prost_build.compile_protos(&proto_files, &include_paths)?;
    
    // Compile with tonic for gRPC services if requested
    if args.with_client || args.with_server {
        println!("Compiling gRPC services...");
        let mut tonic_build = tonic_build::configure()
            .out_dir(&args.output)
            .compile_well_known_types(true);
        
        if !args.with_client {
            tonic_build = tonic_build.build_client(false);
        }
        if !args.with_server {
            tonic_build = tonic_build.build_server(false);
        }
        
        // External type mappings for tonic as well (like ibc-proto-rs)
        tonic_build = tonic_build
            .extern_path(".google.protobuf", "::tendermint_proto::google::protobuf")
            .extern_path(".tendermint", "::tendermint_proto")
            .extern_path(".cosmos", "::cosmos_sdk_proto::cosmos")
            .extern_path(".cosmos.ics23.v1", "::ics23");
        
        tonic_build.compile_protos(&proto_files, &include_paths)?;
    }
    
    println!("Generated Rust code in {}", args.output.display());
    Ok(())
}

fn find_proto_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut proto_files = Vec::new();
    let walker = walkdir::WalkDir::new(dir);
    
    for entry in walker {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "proto" {
                    let path = entry.path().to_path_buf();
                    let path_str = path.to_string_lossy();
                    
                    // Skip problematic googleapis files
                    if path_str.contains("googleapis/google/rpc/error_details.proto") {
                        continue;
                    }
                    
                    // Skip google proto files from cosmos-sdk if they exist in googleapis
                    if path_str.contains("cosmos-sdk/google/") {
                        let relative_path = path.strip_prefix(dir).unwrap();
                        let relative_str = relative_path.to_string_lossy();
                        if relative_str.starts_with("cosmos-sdk/google/") {
                            let googleapis_path = dir.join("googleapis").join(relative_str.strip_prefix("cosmos-sdk/").unwrap());
                            if googleapis_path.exists() {
                                continue; // Skip cosmos-sdk version, use googleapis version
                            }
                        }
                    }
                    
                    // Skip gogoproto from tendermint if it exists in cosmos-sdk
                    if path_str.contains("tendermint/gogoproto/") {
                        let cosmos_sdk_gogoproto = dir.join("cosmos-sdk/gogoproto/gogo.proto");
                        if cosmos_sdk_gogoproto.exists() {
                            continue; // Skip tendermint version, use cosmos-sdk version
                        }
                    }
                    
                    proto_files.push(path);
                }
            }
        }
    }
    
    Ok(proto_files)
}