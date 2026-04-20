defmodule ExFingerprint.Nif do
  @moduledoc false

  # NIF version string must match the version declared in mix.exs.
  @version Mix.Project.config()[:version]

  # GitHub repository hosting the precompiled NIF releases.
  # Binaries are published at:
  #   https://github.com/onnimonni/ex_fingerprint/releases/download/v{version}/
  #     libex_fingerprint_nif-v{version}-nif-{nif_version}-{target}.so.tar.gz
  @base_url "https://github.com/onnimonni/ex_fingerprint/releases/download/v#{@version}"

  use RustlerPrecompiled,
    otp_app: :ex_fingerprint,
    crate: "ex_fingerprint_nif",
    base_url: @base_url,
    # Set EX_FINGERPRINT_BUILD=1 to compile from source (requires Rust + cmake + go + clang).
    force_build: System.get_env("EX_FINGERPRINT_BUILD") in ["1", "true"],
    version: @version,
    targets: [
      "aarch64-unknown-linux-gnu",
      "x86_64-unknown-linux-gnu",
      "aarch64-apple-darwin",
      "x86_64-apple-darwin"
    ]

  @spec request(binary()) :: {:ok, map()} | {:error, term()}
  def request(_payload), do: err()

  @spec profile_metadata() :: {:ok, map()} | {:error, term()}
  def profile_metadata, do: err()

  @spec build_request_plan(binary()) :: {:ok, map()} | {:error, term()}
  def build_request_plan(_payload), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
