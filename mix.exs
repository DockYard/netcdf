defmodule DataParser.MixProject do
  use Mix.Project

  @source_url "https://github.com/DockYard/netcdf"
  @version "0.1.1"

  def project do
    [
      app: :netcdf,
      version: @version,
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      docs: docs(),
      name: "NetCDF",
      description: "NetCDF file format NIF bindings and utilities for Elixir",
      package: package(),
      preferred_cli_env: [
        docs: :docs,
        "hex.publish": :docs
      ]
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
      {:rustler, "~> 0.26.0"},
      {:ex_doc, "~> 0.29.0", only: :docs}
    ]
  end

  defp package do
    [
      maintainers: ["Paulo Valente"],
      licenses: ["MIT"],
      links: %{"GitHub" => @source_url},
      files: [
        "lib",
        "native",
        "mix.exs",
        "LICENSE"
      ],
      exclude_patterns: [
        ~r"native(/[^/]+)*/target"
      ]
    ]
  end

  defp docs do
    [
      main: "readme",
      extras: ["README.md"],
      source_url_pattern: "#{@source_url}/blob/v#{@version}/%{path}#L%{line}"
    ]
  end
end
