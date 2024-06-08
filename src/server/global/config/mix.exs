defmodule ServerOps.MixProject do
  use Mix.Project

  def project do
    [
      app: :server_ops,
      version: "0.1.0",
      elixir: "~> 1.11",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {ServerOps.Application, []}
    ]
  end

  defp deps do
    [
      {:plug_cowboy, "~> 2.5"}
    ]
  end
end
