{
  description = "Cheese Manager - A Rat themed GTK4 File Manager in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      # Compiling Tools
      nativeBuildInputs = with pkgs; [
        pkg-config
        rustc
        cargo
        rustfmt
        rust-analyzer
        clippy
      ];

      # Code libraries
      buildInputs = with pkgs; [
        gtk4
        glib
      ];

      shellHook = ''
        echo -e "\033[1;35m🐀 Welcome to the Sewers: Cheese Manager Dev Shell 🐀\033[0m"
        echo "Rust & GTK4 enviroment ready."
        echo "1. Run 'cargo init' if you haven't already."
        echo "2. Add 'cargo add gtk4 --rename gtk --features v4_12' to your project."
      '';
    };
  };
}
