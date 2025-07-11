{
  description = "Proto Downloader - Tool for downloading protobuf definitions from GitHub repositories";

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

        proto-downloader = pkgs.rustPlatform.buildRustPackage {
          pname = "proto-downloader";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          
          buildInputs = with pkgs; [
            openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];
          
          # Disable tests that require network access for reproducible builds
          doCheck = false;
          
          meta = with pkgs.lib; {
            description = "Tool for downloading protobuf definitions from GitHub repositories";
            homepage = "https://github.com/regen-network/regen";
            license = licenses.asl20;
            maintainers = [ ];
            platforms = platforms.unix;
          };
        };

      in {
        packages = {
          inherit proto-downloader;
          default = proto-downloader;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rust
            
            # Build dependencies
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
            echo "🔧 Proto Downloader Development Environment"
            echo "Rust: $(rustc --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo run -- --help          # Run with help"
            echo "  cargo test                   # Run tests (some require network)"
            echo "  cargo build --release        # Build optimized binary"
            echo ""
            
            # OpenSSL configuration
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
          '';
        };

        # Make it easy to run the tool
        apps.default = {
          type = "app";
          program = "${proto-downloader}/bin/proto-downloader";
        };
      });
} 