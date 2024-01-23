{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.naersk.url = "github:nix-community/naersk";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      rust = pkgs.rust-bin.stable.latest.default.override {
        targets = ["wasm32-unknown-unknown"];
      };
      naersk' = pkgs.callPackage naersk {
        cargo = rust;
        rustc = rust;
      };
      libs = [
        pkgs.xorg.libX11
        pkgs.xorg.libXcursor
        pkgs.xorg.libXrandr
        pkgs.xorg.libXi
        pkgs.vulkan-loader
      ];
      nativeBuildInputs = [
        pkgs.cmake
        pkgs.pkg-config
        pkgs.fontconfig
      ];
    in {
      devShell = pkgs.mkShell {
        nativeBuildInputs =
          [
            pkgs.bashInteractive
            pkgs.gdb
            pkgs.trunk
            pkgs.pandoc
            rust
          ]
          ++ nativeBuildInputs;
        LD_LIBRARY_PATH = "${pkgs.lib.strings.makeLibraryPath libs}";
        RUST_PATH = "${rust}";
      };

      packages.web = let
        pkg = {
          stdenv,
          makeRustPlatform,
          trunk,
          pandoc,
          binaryen,
          wasm-bindgen-cli,
        }: let
          rustPlatformWasm = makeRustPlatform {
            rustc = rust;
            cargo = rust;
          };
        in
          stdenv.mkDerivation {
            pname = "viz";
            version =
              if (self ? shortRev)
              then self.shortRev
              else self.dirtyShortRev;

            src = ./.;

            nativeBuildInputs = [
              trunk
              pandoc
              wasm-bindgen-cli
              binaryen
              rustPlatformWasm.cargoSetupHook
              rustPlatformWasm.cargoBuildHook
            ];

            cargoDeps = rustPlatformWasm.importCargoLock {lockFile = ./Cargo.lock;};

            XDG_CACHE_HOME = "/build/cache";

            buildPhase = ''
              runHook preBuild

              cargo xtask build --release

              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall

              mv target/release/html $out

              runHook postInstall
            '';
          };
      in
        pkgs.callPackage pkg {};

      apps.safety_parabola = flake-utils.lib.mkApp rec {
        drv = naersk'.buildPackage {
          src = ./.;
          targets = ["safety_parabola"];

          nativeBuildInputs = [pkgs.autoPatchelfHook] ++ nativeBuildInputs;
          runtimeDependencies = libs;
        };
        exePath = "/bin/safety_parabola";
      };
    });
}
