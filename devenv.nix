{ pkgs, ... }:

{
  imports = [ ./explicit.generated.deps.nix ];

  packages = [
    pkgs.git
    pkgs.jq
    pkgs.nono
  ];

  enterShell = ''
    echo "Run explicit apply to refresh detected tools and the sandbox plan."
  '';
}
