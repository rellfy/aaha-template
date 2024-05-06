use crate::CONFIG;
use eyre::eyre;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use log::error;
use once_cell::sync::Lazy;
use std::clone::Clone;

static CREDENTIALS: Lazy<Credentials> =
    Lazy::new(|| Credentials::new(CONFIG.smtp_username.clone(), CONFIG.smtp_password.clone()));

pub async fn send_email(recipient: Mailbox, subject: String, body: String) -> eyre::Result<()> {
    let email = Message::builder()
        .from(CONFIG.smtp_email_sender.clone())
        .reply_to(CONFIG.smtp_email_sender.clone())
        .to(recipient)
        .header(ContentType::TEXT_PLAIN)
        .subject(subject)
        .body(body)?;
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&CONFIG.smtp_host)?
        .credentials(CREDENTIALS.clone())
        .build();
    if let Err(e) = mailer.send(email).await {
        error!("failed to send email: {e}");
        return Err(eyre!("failed to send email"));
    };
    Ok(())
}
