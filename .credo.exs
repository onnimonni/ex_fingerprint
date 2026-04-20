%{
  configs: [
    %{
      name: "default",
      files: %{
        included: ["lib/", "config/", "mix.exs"],
        excluded: ["deps/", "_build/"]
      },
      strict: true,
      checks: [
        {Credo.Check.Design.TagTODO, false},
        {Credo.Check.Readability.ModuleDoc, false}
      ]
    }
  ]
}
