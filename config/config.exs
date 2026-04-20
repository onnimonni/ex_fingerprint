import Config

config :ex_fingerprint,
  default_browser_profile: :chrome_latest,
  solver: :chrome,
  profile_aliases: %{
    chrome_latest: :chrome_147
  }

config :rustler,
  otp_app: :ex_fingerprint,
  crate: "ex_fingerprint_nif"
