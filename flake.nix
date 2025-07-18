{
  description = "Regen Network Client - Cosmos SDK development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Latest stable Rust with essential extensions
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "rustfmt"
            "clippy"
          ];
        };

        # Regen Network versions for proto downloads
        regenVersions = {
          ledger = "v6.0.0";
          # Add other repos/versions as needed
        };

        buildInputs = with pkgs; [
          # Core Rust toolchain
          rust

          # Development tools
          cargo-watch # Auto-rebuild on file changes
          cargo-nextest # Better test runner
          cargo-audit # Security vulnerability scanner
          cargo-outdated # Check for outdated dependencies
          cargo-deny # Dependency policy enforcement
          cargo-tarpaulin # Code coverage
          cargo-expand # Macro expansion

          # Additional tools
          just # Command runner
          nixfmt-rfc-style # Nix formatter

          # System dependencies (add as needed)
          pkg-config
          openssl
        ];

      in
      {
        packages = {
          download-regen-protos = pkgs.writeShellScriptBin "download-regen-protos" ''
            echo "🌱 Downloading Regen Network protobuf definitions..."
            echo "Using regen-ledger version: ${regenVersions.ledger}"

            # Use the proto-downloader from its own flake
            PROTO_DOWNLOADER_PATH="crates/regen-rs/tools/proto-downloader"
            OUTPUT_DIR="crates/regen-rs/proto"

            # Run proto-downloader using nix
            cd "$PROTO_DOWNLOADER_PATH" && nix run . -- \
              --output "../../../../$OUTPUT_DIR" \
              --owner regen-network \
              --repo regen-ledger \
              --tag ${regenVersions.ledger} \
              --proto-dir proto

            echo "✅ Regen protos downloaded successfully to $OUTPUT_DIR!"
          '';

          generate-regen-types = pkgs.writeShellScriptBin "generate-regen-types" ''
            echo "🔧 Generating Rust types from Regen Network protobuf definitions..."

            # Use our custom proto-compiler tool (like rs-ibc-proto approach)
            echo "Using proto-compiler tool to generate tonic 0.13 compatible types..."
            cd crates/regen-rs/tools/proto-compiler
            ${rust}/bin/cargo run -- \
              --input ../../proto \
              --output ../../src/generated \
              --with-client

            echo "✅ Regen types generated successfully!"
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs =
            buildInputs
            ++ (with pkgs; [
              # Cosmos SDK / Blockchain development essentials
              protobuf # For protobuf compilation
              buf # Modern protobuf toolchain (will use latest from nixpkgs)

            ])
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
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

        apps = {
          build = {
            type = "app";
            program = "${pkgs.writeScript "build" ''
              #!/bin/sh
              echo "🔨 Building project in dev mode..."
              ${rust}/bin/cargo build --all
              echo "✅ Dev build completed!"
            ''}";
          };

          build-release = {
            type = "app";
            program = "${pkgs.writeScript "build-release" ''
              #!/bin/sh
              echo "🔨 Building project in release mode..."
              ${rust}/bin/cargo build --release --all
              echo "✅ Release build completed!"
            ''}";
          };

          test = {
            type = "app";
            program = "${pkgs.writeScript "test" ''
              #!/bin/sh
              ${pkgs.cargo-nextest}/bin/cargo-nextest nextest run --all
            ''}";
          };

          test-ignored = {
            type = "app";
            program = "${pkgs.writeScript "test-ignored" ''
              #!/bin/sh
              ${pkgs.cargo-nextest}/bin/cargo-nextest nextest run --all -- --include-ignored
            ''}";
          };

          lint = {
            type = "app";
            program = "${pkgs.writeScript "lint" ''
              #!/bin/sh
              ${rust}/bin/cargo clippy --all-targets --all-features -- -D warnings
            ''}";
          };

          format = {
            type = "app";
            program = "${pkgs.writeScript "format" ''
              #!/bin/sh
              echo "🎨 Formatting Rust code..."
              ${rust}/bin/cargo fmt --all

              echo "🎨 Formatting Nix files..."
              ${pkgs.nixfmt-rfc-style}/bin/nixfmt flake.nix
              find . -name "*.nix" -not -path "./target/*" -not -path "./.git/*" -exec ${pkgs.nixfmt-rfc-style}/bin/nixfmt {} \;

              echo "✅ All formatting completed!"
            ''}";
          };

          audit = {
            type = "app";
            program = "${pkgs.writeScript "audit" ''
              #!/bin/sh
              ${pkgs.cargo-audit}/bin/cargo-audit audit
            ''}";
          };

          build-and-check = {
            type = "app";
            program = "${pkgs.writeScript "build-and-check" ''
              #!/bin/sh
              set -e


              echo "🔨 Building project..."
              ${rust}/bin/cargo build --all-targets

              echo "📎 Running clippy (using existing build)..."
              ${rust}/bin/cargo clippy --all-targets --all-features -- -D warnings

              echo "🧪 Running tests (using existing build)..."
              ${pkgs.cargo-nextest}/bin/cargo-nextest nextest run --all

              echo "🧪 Running ignored tests (using existing build)..."
              ${pkgs.cargo-nextest}/bin/cargo-nextest nextest run --all -- --include-ignored

              echo "✅ All checks completed with shared build cache!"
            ''}";
          };

          full-check = {
            type = "app";
            program = "${pkgs.writeScript "full-check" ''
              #!/bin/sh
              set -e


              echo "🚀 Running full project checks..."

              echo "Step 1/4: Checking format..."
              ${rust}/bin/cargo fmt --check

              echo "Step 2/4: Running clippy..."
              ${rust}/bin/cargo clippy --all-targets --all-features -- -D warnings

              echo "Step 3/4: Running ignored tests..."
              ${pkgs.cargo-nextest}/bin/cargo-nextest nextest run --all -- --include-ignored

              echo "Step 4/4: Security audit..."
              ${pkgs.cargo-audit}/bin/cargo-audit audit

              echo "🎉 All checks passed!"
            ''}";
          };
        };

        checks = {
          fmt =
            pkgs.runCommand "cargo-fmt-check"
              {
                src = ./.;
                buildInputs = [ rust ];
              }
              ''
                cd $src
                ${rust}/bin/cargo fmt --check
                touch $out
              '';
        };
      }
    );
}
