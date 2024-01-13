{
  description = "CSS-2324 Bonus exercise flake";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustfmt
          rustc
          cargo
          sccache
          rust-analyzer
          clippy
          openssl
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        shellHook = ''
          export RUSTC_WRAPPER=sccache
        '';
      };
    };
}
