defmodule MailParser.MixProject do
  use Mix.Project

  @version "0.3.4"
  @repo_url "https://github.com/kloeckner-i/mail_parser"

  def project do
    [
      app: :mail_parser,
      version: @version,
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      docs: docs(),
      description: "NIF binding of mail_parser using Rustler",
      package: package()
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
      {:rustler, ">= 0.0.0", optional: true},
      {:rustler_precompiled, "~> 0.5"},
      {:ex_doc, ">= 0.0.0", only: :dev}
    ]
  end

  defp docs do
    [
      main: "MailParser",
      extras: ["CHANGELOG.md"],
      skip_undefined_reference_warnings_on: ["CHANGELOG.md"],
      source_ref: "v#{@version}",
      source_url: @repo_url
    ]
  end

  defp package do
    [
      files: [
        "lib",
        "native/mail_parser_nif/Cargo.toml",
        "native/mail_parser_nif/Cargo.lock",
        "native/mail_parser_nif/src",
        "native/mail_parser_nif/.cargo",
        "checksum-*.exs",
        "mix.exs",
        "README.md",
        "CHANGELOG.md",
        "LICENSE-APACHE",
        "LICENSE-MIT"
      ],
      maintainers: ["kloeckner-i"],
      licenses: ["MIT", "Apache-2.0"],
      links: %{"GitHub" => @repo_url}
    ]
  end
end
