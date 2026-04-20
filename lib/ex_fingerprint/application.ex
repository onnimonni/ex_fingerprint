defmodule ExFingerprint.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      {ExFingerprint.ProfileRegistry, []},
      {ExFingerprint.CookieJar, []},
      {ExFingerprint.Rotator, []},
      {Task.Supervisor, name: ExFingerprint.Solver.TaskSupervisor},
      {ExFingerprint.Solver.Registry, []}
    ]

    Supervisor.start_link(children, strategy: :one_for_one, name: ExFingerprint.Supervisor)
  end
end
