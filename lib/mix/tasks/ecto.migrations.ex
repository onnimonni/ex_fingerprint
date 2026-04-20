defmodule Mix.Tasks.Ecto.Migrations do
  use Mix.Task

  @shortdoc "No-op migration task for projects without Ecto"

  @moduledoc """
  Compatibility task for verification environments that expect
  `mix ecto.migrations`.

  This project has no Ecto repos or migrations yet, so the task returns an
  empty result.
  """

  @impl Mix.Task
  def run(_args) do
    Mix.shell().info("[]")
  end
end
