defmodule MailParser do
  @moduledoc """
  A NIF for extracting attachments from mails in the Internet Message Format
  standard.
  """

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :mail_parser,
    crate: :mail_parser_nif,
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_FORCE_BUILD") in ["1", "true"],
    version: version

  alias __MODULE__.Attachment

  @spec parse(String.t()) :: {:ok, [Attachment.t()]} | {:error, term}
  def parse(_input), do: :erlang.nif_error(:nif_not_loaded)
end
