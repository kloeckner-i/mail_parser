defmodule MailParser.Attachment do
  @moduledoc """
  A message attachment.
  """

  @type t :: %__MODULE__{
          name: String.t(),
          content_bytes: binary,
          content_type: String.t() | nil
        }

  defstruct [:name, :content_type, :content_bytes]
end
