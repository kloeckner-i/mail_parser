use mail_parser::{BodyPart, Message, MessageAttachment, MessagePart};
use rustler::{Atom, Binary, Env, Error, NifResult, NifStruct, OwnedBinary, Term};
use rustler::{Decoder, Encoder};

mod atoms {
    rustler::atoms! {
        ok
    }
}

#[derive(Clone, Debug, NifStruct)]
#[module = "MailParser.Attachment"]
struct Attachment {
    name: String,
    content_type: Option<String>,
    content_bytes: ContentBytes,
}

impl Attachment {
    fn create<'x>(part: &'x impl BodyPart<'x>) -> Attachment {
        let name = part.get_attachment_name().unwrap_or("untitled").to_string();
        let content_bytes = ContentBytes::new(part.get_contents());

        let content_type = part.get_content_type().map(|content_type| {
            let roottype = content_type.get_type();

            match content_type.get_subtype() {
                Some(subtype) => format!("{roottype}/{subtype}"),
                None => roottype.to_string(),
            }
        });

        Attachment {
            name,
            content_bytes,
            content_type,
        }
    }
}

#[derive(Clone, Debug)]
struct ContentBytes(Vec<u8>);

impl ContentBytes {
    fn new(content_bytes: &[u8]) -> Self {
        ContentBytes(content_bytes.to_vec())
    }
}

impl Encoder for ContentBytes {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let mut owned_binary = OwnedBinary::new(self.0.len()).expect("allocation failed");
        owned_binary.as_mut_slice().copy_from_slice(&self.0);
        Binary::from_owned(owned_binary, env).encode(env)
    }
}
impl Decoder<'_> for ContentBytes {
    fn decode(term: Term) -> NifResult<ContentBytes> {
        Ok(Self(term.to_binary().to_vec()))
    }
}

fn extract_attachments(message: &Message) -> Vec<Attachment> {
    let mut acc = Vec::new();

    for attachment in message.get_attachments() {
        match attachment {
            MessagePart::Text(text) | MessagePart::Html(text) => {
                acc.push(Attachment::create(text));
            }

            MessagePart::Binary(blob) | MessagePart::InlineBinary(blob) => {
                acc.push(Attachment::create(blob));
            }

            MessagePart::Message(attached_message) => {
                match &attached_message.body {
                    MessageAttachment::Parsed(message) => {
                        acc.extend(extract_attachments(message));
                    }
                    MessageAttachment::Raw(_raw_bytes) => {
                        if let Some(message) = attached_message.parse_raw() {
                            acc.extend(extract_attachments(&message));
                        }
                    }
                }

                acc.push(Attachment::create(attached_message));
            }

            MessagePart::Multipart(_multipart_message) => (),
        }
    }

    acc
}

#[rustler::nif]
fn extract_nested_attachments(raw_message: &str) -> NifResult<(Atom, Vec<Attachment>)> {
    match Message::parse(raw_message.as_bytes()) {
        Some(message) => Ok((atoms::ok(), extract_attachments(&message))),
        None => Err(Error::Atom("error")),
    }
}

rustler::init!("Elixir.MailParser", [extract_nested_attachments]);
