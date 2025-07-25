defmodule AnomaSdk.MixProject do
  use Mix.Project

  def project do
    [
      app: :anoma_sdk,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.36.1", runtime: false},
      {:typed_struct, "~> 0.3.0"}
    ]
  end
end
