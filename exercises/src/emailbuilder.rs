#[derive(Debug)]
#[allow(dead_code)]
struct Email {
    to: String,
    subject: String,
    body: Option<String>,
    cc: Vec<String>,
}

#[allow(dead_code)]
#[derive(Default)]
struct EmailBuilder {
    to: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    cc: Vec<String>,
}
#[allow(dead_code)]
impl EmailBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }
    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }
    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    fn cc(mut self, addr: impl Into<String>) -> Self {
        self.cc.push(addr.into());
        self
    }
    fn build(self) -> Result<Email, String> {
        let to = self
            .to
            .filter(|s| !s.is_empty())
            .ok_or("'to' is required")?;
        let subject = self
            .subject
            .filter(|s| !s.is_empty())
            .ok_or("'subject' is required")?;
        Ok(Email {
            to,
            subject,
            body: self.body,
            cc: self.cc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_email() {
        let email = EmailBuilder::new()
            .to("alice@example.com")
            .subject("Hello")
            .build();
        assert!(email.is_ok());
    }
    #[test]
    fn missing_to_fails() {
        let email = EmailBuilder::new().subject("Hello").build();
        assert!(email.is_err());
    }
}
