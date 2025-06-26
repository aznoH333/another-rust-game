let 
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "music-reader-env";
    buildInputs = [ 
      cargo
      rustc
      gcc
      alsa-lib
      alsa-lib.dev
      nix
      udev
      ];
nativeBuildInputs = with pkgs; [ pkg-config ];

  }
