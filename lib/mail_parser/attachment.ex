defmodule MailParser.Attachment do
  @moduledoc false

  @type t :: %__MODULE__{
          name: String.t(),
          content_bytes: binary,
          content_type: String.t()
        }

  defstruct [:name, :content_type, :content_bytes]
end
