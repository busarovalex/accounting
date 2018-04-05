use lettre::EmailTransport;
use lettre_email::{EmailBuilder, MimeMessage, PartBuilder};
use lettre::smtp;
use lettre::smtp::authentication::Credentials;

use error::{Error, ErrorKind};
use config::Config;

pub struct EmailSender {
    from: String,
    host: String,
    username: String,
    password: String,
}

impl EmailSender {
    pub fn from_config(config: &Config) -> Result<EmailSender, Error> {
        let from = config.email_from.clone().ok_or(not_set_up("email_from"))?;
        let host = config
            .email_smtp_host
            .clone()
            .ok_or(not_set_up("email_smtp_host"))?;
        let username = config
            .email_smtp_credential_username
            .clone()
            .ok_or(not_set_up("email_smtp_credential_username"))?;
        let password = config
            .email_smtp_credential_password
            .clone()
            .ok_or(not_set_up("email_smtp_credential_password"))?;
        Ok(EmailSender {
            from,
            host,
            username,
            password,
        })
    }

    pub fn send(&self, data: String, email: &str) -> Result<(), Error> {
        debug!("sendign email to \"{}\"", email);

        let mut email = EmailBuilder::new()
            .to(email)
            .from(self.from.clone())
            .subject("Отчет")
            .text("Высылаем желаемый отчет");
        email.set_message_type(::lettre_email::MimeMultipartType::Mixed);
        email.add_child(child(data));
        let email = email.build()?;
        trace!("creating mailer");

        let mut mailer = smtp::SmtpTransport::simple_builder(self.host.clone())?
            .credentials(Credentials::new(
                self.username.clone(),
                self.password.clone(),
            ))
            .build();

        trace!("sending email");

        mailer.send(&email)?;

        Ok(())
    }
}

fn child(data: String) -> MimeMessage {
    let encoded: String = ::base64::encode(data.as_bytes());
    PartBuilder::new()
        .body(encoded)
        .header((
            "Content-Disposition",
            format!("attachment; filename=\"Отчет.html\""),
        ))
        .header(("Content-Type", ::mime::TEXT_HTML.to_string()))
        .header(("Content-Transfer-Encoding", "base64"))
        .build()
}

fn not_set_up(property: &str) -> Error {
    ErrorKind::EmailNotSetUp(property.to_owned()).into()
}
