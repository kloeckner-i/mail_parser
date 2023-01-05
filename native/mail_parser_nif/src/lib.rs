use mail_parser::{Message, MessagePart, MimeHeaders};
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

impl From<&MessagePart<'_>> for Attachment {
    fn from(part: &MessagePart) -> Self {
        let name = part.attachment_name().unwrap_or("untitled").to_string();
        let content_bytes = ContentBytes::new(part.contents());

        let content_type = part.content_type().map(|content_type| {
            let roottype = content_type.ctype();

            match content_type.subtype() {
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

fn get_attachments(message: &Message) -> Vec<Attachment> {
    message
        .attachments()
        .flat_map(|attachment| match attachment.message() {
            Some(nested_message) => get_attachments(nested_message),
            None => Vec::from([attachment.into()]),
        })
        .collect()
}

#[rustler::nif]
fn extract_nested_attachments(raw_message: &str) -> NifResult<(Atom, Vec<Attachment>)> {
    match Message::parse(raw_message.as_bytes()) {
        Some(message) => Ok((atoms::ok(), get_attachments(&message))),
        None => Err(Error::Atom("error")),
    }
}

rustler::init!("Elixir.MailParser", [extract_nested_attachments]);
