defmodule ExFingerprint.Nif do
  @moduledoc false
  use Rustler, otp_app: :ex_fingerprint, crate: "ex_fingerprint_nif"

  @spec request(binary()) :: {:ok, map()} | {:error, term()}
  def request(_payload), do: err()

  @spec profile_metadata() :: {:ok, map()} | {:error, term()}
  def profile_metadata, do: err()

  @spec build_request_plan(binary()) :: {:ok, map()} | {:error, term()}
  def build_request_plan(_payload), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
