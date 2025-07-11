{
  description = "Regen Network Client - Cosmos SDK development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        # Latest stable Rust with essential extensions
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Regen Network versions for proto downloads
        regenVersions = {
          ledger = "v6.0.0";
          # Add other repos/versions as needed
        };

      in {
        packages = {
          download-regen-protos = pkgs.writeShellScriptBin "download-regen-protos" ''
            echo "🌱 Downloading Regen Network protobuf definitions..."
            echo "Using regen-ledger version: ${regenVersions.ledger}"
            
            # Use the proto-downloader from its own flake
            PROTO_DOWNLOADER_PATH="crates/regen-types/tools/proto-downloader"
            OUTPUT_DIR="crates/regen-types/proto/regen"
            
            # Create the output directory
            mkdir -p "$OUTPUT_DIR"
            
            # Run proto-downloader using nix
            cd "$PROTO_DOWNLOADER_PATH" && nix run . -- \
              --output "../../../../$OUTPUT_DIR" \
              --owner regen-network \
              --repo regen-ledger \
              --tag ${regenVersions.ledger} \
              --proto-dir proto
            
            echo "✅ Regen protos downloaded successfully to $OUTPUT_DIR!"
          '';
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rust
            
            # Cosmos SDK / Blockchain development essentials
            protobuf  # For protobuf compilation
            pkg-config
            openssl
            
            # Development tools
            cargo-watch
            clippy
            rustfmt
            
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # macOS specific frameworks
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          shellHook = ''
            echo "🌱 Regen Network Client Development Environment"
            echo "Rust: $(rustc --version)"
            echo "Protobuf: $(protoc --version)"
            echo ""
            
            # Environment setup for cosmos-sdk development
            export RUST_LOG=info
            export RUST_BACKTRACE=1
            export PROTOC="${pkgs.protobuf}/bin/protoc"
            export PROTOC_INCLUDE="${pkgs.protobuf}/include"
            
            # OpenSSL configuration
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
          '';
        };
      });
} 