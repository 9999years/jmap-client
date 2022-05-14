use std::collections::HashMap;

use crate::{
    core::{
        request::ResultReference,
        set::{from_timestamp, Create},
    },
    Set,
};

use super::{
    Email, EmailAddress, EmailAddressGroup, EmailBodyPart, EmailBodyValue, EmailHeader, Header,
    HeaderValue,
};

impl Email<Set> {
    pub fn mailbox_ids<T, U>(&mut self, mailbox_ids: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.mailbox_ids = Some(mailbox_ids.into_iter().map(|s| (s.into(), true)).collect());
        self.mailbox_ids_ref = None;
        self
    }

    pub fn mailbox_ids_ref(&mut self, reference: ResultReference) -> &mut Self {
        self.mailbox_ids_ref = reference.into();
        self.mailbox_ids = None;
        self
    }

    pub fn mailbox_id(&mut self, mailbox_id: &str, set: bool) -> &mut Self {
        self.mailbox_ids = None;
        self.patch.insert(format!("mailboxIds/{}", mailbox_id), set);
        self
    }

    pub fn keywords<T, U>(&mut self, keywords: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.keywords = Some(keywords.into_iter().map(|s| (s.into(), true)).collect());
        self
    }

    pub fn keyword(&mut self, keyword: &str, set: bool) -> &mut Self {
        self.keywords = None;
        self.patch.insert(format!("keywords/{}", keyword), set);
        self
    }

    pub fn message_id<T, U>(&mut self, message_id: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.message_id = Some(message_id.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn in_reply_to<T, U>(&mut self, in_reply_to: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.in_reply_to = Some(in_reply_to.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn references<T, U>(&mut self, references: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.references = Some(references.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn sender<T, U>(&mut self, sender: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.sender = Some(sender.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn from<T, U>(&mut self, from: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.from = Some(from.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn to<T, U>(&mut self, to: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.to = Some(to.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn cc<T, U>(&mut self, cc: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.cc = Some(cc.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn bcc<T, U>(&mut self, bcc: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.bcc = Some(bcc.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn reply_to<T, U>(&mut self, reply_to: T) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<EmailAddress>,
    {
        self.reply_to = Some(reply_to.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn subject(&mut self, subject: impl Into<String>) -> &mut Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn sent_at(&mut self, sent_at: i64) -> &mut Self {
        self.sent_at = Some(from_timestamp(sent_at));
        self
    }

    pub fn body_structure(&mut self, body_structure: EmailBodyPart) -> &mut Self {
        self.body_structure = Some(body_structure.into());
        self
    }

    pub fn body_value(&mut self, id: String, body_value: impl Into<EmailBodyValue>) -> &mut Self {
        self.body_values
            .get_or_insert_with(HashMap::new)
            .insert(id, body_value.into());
        self
    }

    pub fn text_body(&mut self, text_body: EmailBodyPart) -> &mut Self {
        self.text_body.get_or_insert_with(Vec::new).push(text_body);
        self
    }

    pub fn html_body(&mut self, html_body: EmailBodyPart) -> &mut Self {
        self.html_body.get_or_insert_with(Vec::new).push(html_body);
        self
    }

    pub fn attachment(&mut self, attachment: EmailBodyPart) -> &mut Self {
        self.attachments
            .get_or_insert_with(Vec::new)
            .push(attachment);
        self
    }

    pub fn header(&mut self, header: Header, value: impl Into<HeaderValue>) -> &mut Self {
        self.headers.insert(header, Some(value.into()));
        self
    }
}

impl Create for Email<Set> {
    fn new(_create_id: Option<usize>) -> Email<Set> {
        Email {
            _create_id,
            _state: Default::default(),
            id: Default::default(),
            blob_id: Default::default(),
            thread_id: Default::default(),
            mailbox_ids: Default::default(),
            mailbox_ids_ref: Default::default(),
            keywords: Default::default(),
            size: Default::default(),
            received_at: Default::default(),
            message_id: Default::default(),
            in_reply_to: Default::default(),
            references: Default::default(),
            sender: Default::default(),
            from: Default::default(),
            to: Default::default(),
            cc: Default::default(),
            bcc: Default::default(),
            reply_to: Default::default(),
            subject: Default::default(),
            sent_at: Default::default(),
            body_structure: Default::default(),
            body_values: Default::default(),
            text_body: Default::default(),
            html_body: Default::default(),
            attachments: Default::default(),
            has_attachment: Default::default(),
            preview: Default::default(),
            headers: Default::default(),
            patch: Default::default(),
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl EmailBodyPart {
    pub fn new() -> EmailBodyPart<Set> {
        EmailBodyPart {
            part_id: None,
            blob_id: None,
            size: None,
            headers: None,
            name: None,
            type_: None,
            charset: None,
            disposition: None,
            cid: None,
            language: None,
            location: None,
            sub_parts: None,
            header: None,
            _state: Default::default(),
        }
    }
}

impl EmailBodyPart<Set> {
    pub fn part_id(mut self, part_id: impl Into<String>) -> Self {
        self.part_id = Some(part_id.into());
        self
    }

    pub fn blob_id(mut self, blob_id: impl Into<String>) -> Self {
        self.blob_id = Some(blob_id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.type_ = Some(content_type.into());
        self
    }

    pub fn content_id(mut self, content_id: impl Into<String>) -> Self {
        self.cid = Some(content_id.into());
        self
    }

    pub fn content_language<T, U>(mut self, content_language: T) -> Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.language = Some(content_language.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn content_location(mut self, content_location: impl Into<String>) -> Self {
        self.location = Some(content_location.into());
        self
    }

    pub fn sub_part(mut self, sub_part: EmailBodyPart) -> Self {
        self.sub_parts.get_or_insert_with(Vec::new).push(sub_part);
        self
    }
}

impl From<String> for EmailBodyValue {
    fn from(value: String) -> Self {
        EmailBodyValue {
            value,
            is_encoding_problem: None,
            is_truncated: None,
            _state: Default::default(),
        }
    }
}

impl From<&str> for EmailBodyValue {
    fn from(value: &str) -> Self {
        EmailBodyValue {
            value: value.to_string(),
            is_encoding_problem: None,
            is_truncated: None,
            _state: Default::default(),
        }
    }
}

impl EmailAddress {
    pub fn new(email: String) -> EmailAddress<Set> {
        EmailAddress {
            _state: Default::default(),
            name: None,
            email,
        }
    }
}

impl EmailAddress<Set> {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

impl From<String> for EmailAddress {
    fn from(email: String) -> Self {
        EmailAddress {
            _state: Default::default(),
            name: None,
            email,
        }
    }
}

impl From<(String, String)> for EmailAddress {
    fn from(parts: (String, String)) -> Self {
        EmailAddress {
            _state: Default::default(),
            name: parts.0.into(),
            email: parts.1,
        }
    }
}

impl From<&str> for EmailAddress {
    fn from(email: &str) -> Self {
        EmailAddress {
            _state: Default::default(),
            name: None,
            email: email.to_string(),
        }
    }
}

impl From<(&str, &str)> for EmailAddress {
    fn from(parts: (&str, &str)) -> Self {
        EmailAddress {
            _state: Default::default(),
            name: parts.0.to_string().into(),
            email: parts.1.to_string(),
        }
    }
}

impl EmailAddressGroup {
    pub fn new() -> EmailAddressGroup<Set> {
        EmailAddressGroup {
            _state: Default::default(),
            name: None,
            addresses: Vec::new(),
        }
    }
}

impl EmailAddressGroup<Set> {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn address(mut self, address: impl Into<EmailAddress>) -> Self {
        self.addresses.push(address.into());
        self
    }
}

impl EmailHeader {
    pub fn new(name: String, value: String) -> EmailHeader<Set> {
        EmailHeader {
            _state: Default::default(),
            name,
            value,
        }
    }
}
