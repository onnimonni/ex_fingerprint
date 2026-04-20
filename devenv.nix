{ pkgs, ... }:

{
  imports = [ ./explicit.generated.deps.nix ];

  env = {
    LLVM_COV = "${pkgs.llvm}/bin/llvm-cov";
    LLVM_PROFDATA = "${pkgs.llvm}/bin/llvm-profdata";
  };

  packages = [
    pkgs.git
    pkgs.jq
    pkgs.llvm
    pkgs.nono
  ];

  enterShell = ''
    echo "Run explicit apply to refresh detected tools and the sandbox plan."
  '';
}
