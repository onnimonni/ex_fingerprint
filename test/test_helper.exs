nif_enabled? = System.get_env("STUFFIX_NATIVE_FINGERPRINT") == "true"

exclude = if nif_enabled?, do: [], else: [:nif_required]

ExUnit.start(exclude: exclude)
