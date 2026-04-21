defmodule ExUndercover.ProfileRegistry do
  use GenServer

  alias ExUndercover.Profile
  alias ExUndercover.Transport.ProfileSync

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  @impl true
  def init(state) do
    # Call known_profiles/0 first to ensure ExUndercover.Profile and its submodules
    # are loaded (registers profile ID atoms like :chrome_147) before any
    # String.to_existing_atom call in the alias resolution path.
    known = Profile.known_profiles()

    latest_profile =
      case ProfileSync.latest_alias_target(:chrome_latest) do
        {:ok, target} -> Profile.resolve(target)
        _ -> Profile.chrome_latest()
      end

    profiles =
      known
      |> Enum.reject(&(&1 == :chrome_latest))
      |> Enum.reduce(%{chrome_latest: latest_profile}, fn profile_id, acc ->
        Map.put(acc, profile_id, Profile.resolve(profile_id))
      end)

    {:ok, Map.put(state, :profiles, profiles)}
  end
end
