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
        {Credo.Check.Readability.ModuleDoc, false},
        # Style opinions not enforced in this library
        {Credo.Check.Design.AliasUsage, false},
        {Credo.Check.Readability.AliasOrder, false},
        {Credo.Check.Readability.LargeNumbers, false},
        {Credo.Check.Refactor.RedundantWithClauseResult, false},
        {Credo.Check.Refactor.CyclomaticComplexity, false},
        {Credo.Check.Refactor.FunctionArity, false},
        {Credo.Check.Consistency.ParameterPatternMatching, false},
        {Credo.Check.Readability.PipeIntoAnonymousFunctions, false}
      ]
    }
  ]
}
