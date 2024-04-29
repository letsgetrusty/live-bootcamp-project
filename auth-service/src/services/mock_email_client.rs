use crate::domain::{Email, EmailClient};

pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> Result<(), String> {
        println!(
            "Sending email to {} with subject: {} and content: {}",
            recipient.as_ref(),
            subject,
            content
        );

        Ok(())
    }
}
