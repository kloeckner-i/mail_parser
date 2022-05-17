defmodule MailParser do
  @moduledoc """
  A NIF for extracting attachments from mails in the Internet Message Format
  standard.
  """

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :mail_parser,
    crate: :mailparser,
    base_url: "https://github.com/adriankumpf/mail_parser/releases/download/v0.1.0",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_FORCE_BUILD") in ["1", "true"],
    version: version

  alias __MODULE__.Attachment

  @spec parse(String.t()) :: {:ok, [Attachment.t()]} | {:error, term}
  def parse(_input), do: :erlang.nif_error(:nif_not_loaded)
end
