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

      in {
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