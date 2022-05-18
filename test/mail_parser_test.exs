defmodule MailParserTest do
  use ExUnit.Case

  doctest MailParser, except: [extract_nested_attachments: 1]

  test "extracts attachments from raw message" do
    raw_message = File.read!("test/fixtures/example.txt")

    assert {:ok,
            [
              %MailParser.Attachment{
                name: "Best 340 Klöckner FL-Stahl.pdf",
                content_type: "application/pdf",
                content_bytes: pdf_content_bytes
              },
              %MailParser.Attachment{
                name: "smime.p7s",
                content_type: "application/x-pkcs7-signature",
                content_bytes: "cmVkYWN0ZWQ="
              }
            ]} = MailParser.extract_nested_attachments(raw_message)

    assert Base.decode64!(pdf_content_bytes) == File.read!("test/fixtures/sample.pdf")
  end
end
