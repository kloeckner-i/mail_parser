use mail_parser::*;
use rustler::{Atom, Error, NifResult, NifStruct};

#[rustler::nif]
fn parse(input: &str) -> NifResult<(Atom, Vec<Attachment>)> {
    rustler::atoms! { ok, error, }

    match Message::parse(input.as_bytes()) {
        Some(message) => Ok((ok(), extract_attachments(&message))),
        None => Err(Error::Atom("error")),
    }
}

#[derive(Clone, Debug, NifStruct)]
#[module = "MailParser.Attachment"]
struct Attachment {
    name: String,
    content_type: Option<String>,
    content_bytes: String,
}

impl Attachment {
    fn create<'x>(part: &'x impl BodyPart<'x>) -> Attachment {
        let name = part
            .get_attachment_name()
            .unwrap_or("untitled.txt")
            .to_string();

        let content_bytes = base64::encode(part.get_contents());

        let content_type = part.get_content_type().map(|ct| {
            let roottype = ct.get_type();
            let subtype = ct.get_subtype().unwrap_or("unknown");
            format!("{roottype}/{subtype}")
        });

        Attachment {
            name,
            content_bytes,
            content_type,
        }
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
                        acc.extend(extract_attachments(message.as_ref()));
                    }
                    MessageAttachment::Raw(_) => {
                        acc.extend(extract_attachments(&attached_message.parse_raw().unwrap()));
                    }
                }

                acc.push(Attachment::create(attached_message));
            }

            MessagePart::Multipart(_multipart_message) => (),
        }
    }

    acc
}

rustler::init!("Elixir.MailParser", [parse]);
