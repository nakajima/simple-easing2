{
	description = "simple-easing2";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem (system:
			let pkgs = import nixpkgs { inherit system; };
			in {
				devShell = pkgs.mkShell {
					name = "simple-easing2";
					buildInputs = with pkgs; [
						cargo
						rustc
						rustfmt
						rust-analyzer
					];
				};
			}
		);
}
