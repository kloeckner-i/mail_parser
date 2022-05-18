# Changelog

## [Unreleased]

## [0.3.0] - 2022-05-18

- Return `:content_bytes` as Erlang binary instead of base64 encoded string
- Eliminate call to `unwrap()`
- Publish package to hex

## [0.2.0] - 2022-05-17

- Disable `mail_parser`s default features
- Rename `MailParser.parse` to `MailParser.extract_nested_attachments`
- Rename NIF package
- Add checksum file

## [0.1.0] - 2022-05-17

[unreleased]: https://github.com/kloeckner-i/mail_parser/compare/v0.3.0...HEAD
[0.2.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.3.0
[0.2.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.2.0
[0.1.0]: https://github.com/kloeckner-i/mail_parser/releases/tag/v0.1.0
