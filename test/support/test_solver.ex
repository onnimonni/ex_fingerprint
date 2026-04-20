defmodule ExFingerprint.TestSupport.TestSolver do
  @behaviour ExFingerprint.Solver

  @impl true
  def solve(_url, _opts) do
    {:ok,
     %{
       browser: :chrome,
       cookies: [%{name: "solver", value: "ok"}],
       current_url: "http://solver.local/success",
       title: "Solved",
       body_text: "solver ok",
       headless: true
     }}
  end
end
