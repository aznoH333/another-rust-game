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
      nix
alsa-lib
              udev
              #NOTE Add more deps
              vulkan-loader
             
glfw-wayland 
xorg.libX11
        xorg.libX11.dev
        xorg.libXcursor
        xorg.libXi
        xorg.libXinerama
        xorg.libXrandr            
          vulkan-headers
          libxkbcommon
      ];
nativeBuildInputs = with pkgs; [ 
udev alsa-lib vulkan-loader
    libxkbcommon wayland # To use the wayland feature
glfw-wayland
pkg-config
];
# Make sure the graphics libraries are findable in our dev shell.
  shellHook = ''
    export LD_LIBRARY_PATH="${lib.makeLibraryPath [
      "/run/opengl-driver"
      "/run/opengl-driver-32"
      libGL
      vulkan-loader
    ]}:$LD_LIBRARY_PATH"
  '';
  }
