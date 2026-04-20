defmodule ExFingerprint.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_fingerprint,
      version: "0.1.0",
      elixir: "~> 1.16",
      elixirc_paths: elixirc_paths(Mix.env()),
      test_coverage: test_coverage(),
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      mod: {ExFingerprint.Application, []},
      extra_applications: [:logger]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_env), do: ["lib"]

  defp test_coverage do
    [
      summary: [threshold: 90],
      ignore_modules: [
        ~r/^Mix\.Tasks\./,
        ExFingerprint,
        ExFingerprint.Capture.ClientHello,
        ExFingerprint.CookieJar,
        ExFingerprint.CookieJar.Cookie,
        ExFingerprint.Debug,
        ExFingerprint.Nif,
        ExFingerprint.Proton,
        ExFingerprint.Profile.Chrome147,
        ExFingerprint.ProfileRegistry,
        ExFingerprint.Runtime,
        ExFingerprint.Solver,
        ExFingerprint.Solver.Chrome,
        ExFingerprint.Solver.Chrome.CDPConnection,
        ExFingerprint.TestSupport.CountingSolver,
        ExFingerprint.TestSupport.HTTPServer,
        ExFingerprint.Transport.ProfileSync,
        ExFingerprint.WireGuard.PolicyRouting,
        ExFingerprint.WireGuard.Config,
        ExFingerprint.WireGuard.InterfaceConfig,
        ExFingerprint.WireGuard.Manager
      ]
    ]
  end

  defp deps do
    [
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:jason, "~> 1.4"},
      {:rustler, "~> 0.36", optional: true},
      {:rustler_precompiled, "~> 0.7"},
      {:websockex, "~> 0.4"},
      {:wireguardex, "~> 0.4"}
    ]
  end
end
