# Architecture

## Migration map from `fauxbrowser`

This project keeps the same high-level responsibilities as
`onnimonni/fauxbrowser`, but moves them into different layers.

### Network egress

`fauxbrowser`:

- userspace WireGuard
- gVisor netstack
- custom dialer inside the proxy binary

`ex_fingerprint`:

- kernel WireGuard device
- interface lifecycle managed from Elixir with `wireguardex`
- routing handled by the OS
- Rust transport opens normal sockets

### Browser impersonation

`fauxbrowser`:

- Go transport
- `tls-client`
- browser profiles embedded in the proxy process

`ex_fingerprint`:

- Rustler NIF transport
- versioned browser profiles selected from Elixir
- Rust owns TLS + HTTP/2 fingerprint execution

### Proxy/orchestration layer

`fauxbrowser`:

- plaintext h1/h2c proxy listener
- response inspection triggers rotation
- solver cookies and host-level diagnostics in Go

`ex_fingerprint`:

- Elixir supervision and orchestration
- future Phoenix/Bandit or plain TCP listener if proxy mode is still desired
- host policy, retries, escalation, and telemetry in Elixir

### Solver escalation

`fauxbrowser`:

- chromedp launched on demand

`ex_fingerprint`:

- explicit solver behaviour in Elixir
- pluggable real-Chrome backend later

## Initial module split

- `ExFingerprint.Client`
  Elixir API boundary for request execution.
- `ExFingerprint.Profile`
  Versioned browser profile selection.
- `ExFingerprint.WireGuard.Manager`
  Kernel WireGuard lifecycle only.
- `ExFingerprint.Solver`
  Real browser escalation boundary.
- `ExFingerprint.Nif`
  Native transport boundary.

## Important non-goal

This project does not embed a VPN data plane inside the HTTP client.

That is the main departure from `fauxbrowser`, and it is intentional.
