# MailParser

[![CI](https://github.com/kloeckner-i/mail_parser/actions/workflows/ci.yml/badge.svg)](https://github.com/kloeckner-i/mail_parser/actions/workflows/ci.yml)
[![Build precompiled NIFs](https://github.com/kloeckner-i/mail_parser/actions/workflows/release.yml/badge.svg)](https://github.com/kloeckner-i/mail_parser/actions/workflows/release.yml)
[![Docs](https://img.shields.io/badge/hex-docs-green.svg?style=flat)](https://hexdocs.pm/mail_parser)
[![Hex.pm](https://img.shields.io/hexpm/v/mail_parser?color=%23714a94)](http://hex.pm/packages/mail_parser)

NIF binding of [mail_parser](https://docs.rs/mail-parser/latest/mail_parser/) using [Rustler](https://github.com/rusterlium/rustler).

## Installation

The package can be installed by adding `mail_parser` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:mail_parser, "~> 0.3.0"}
  ]
end
```

## Forcing compilation

By default **you don't need Rust installed** because the library will try to download a precompiled NIF file. In case you want to force compilation set the `FORCE_BUILD` environment variable to `true`.

You also need to add Rustler to your dependencies when you want to force the compilation:

```elixir
def deps do
  [
    {:mail_parser, "~> 0.3.0"}
    {:rustler, ">= 0.0.0", optional: true}
  ]
end
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
