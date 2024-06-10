use color_eyre::eyre::Result;

use reqwest::{Client, Url};
use secrecy::{ExposeSecret, Secret};

use crate::domain::{Email, EmailClient};

pub struct PostmarkEmailClient {
    http_client: Client,
    base_url: String,
    sender: Email,
    authorization_token: Secret<String>,
}

impl PostmarkEmailClient {
    pub fn new(
        base_url: String,
        sender: Email,
        authorization_token: Secret<String>,
        http_client: Client,
    ) -> Self {
        Self {
            http_client,
            base_url,
            sender,
            authorization_token,
        }
    }
}

#[async_trait::async_trait]
impl EmailClient for PostmarkEmailClient {
    #[tracing::instrument(name = "Sending email", skip_all)]
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()> {
        let base = Url::parse(&self.base_url)?;
        let url = base.join("/email")?;

        let request_body = SendEmailRequest {
            from: self.sender.as_ref().expose_secret(),
            to: recipient.as_ref().expose_secret(),
            subject,
            html_body: content,
            text_body: content,
            message_stream: MESSAGE_STREAM,
        };

        let request = self
            .http_client
            .post(url)
            .header(
                POSTMARK_AUTH_HEADER,
                self.authorization_token.expose_secret(),
            )
            .json(&request_body);

        request.send().await?.error_for_status()?;

        Ok(())
    }
}

const MESSAGE_STREAM: &str = "outbound";
const POSTMARK_AUTH_HEADER: &str = "X-Postmark-Server-Token";

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
    message_stream: &'a str,
}

#[cfg(test)]
mod tests {
    use crate::utils::constants::test;

    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::{any, header, header_exists, method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    use super::PostmarkEmailClient;

    fn subject() -> String {
        Sentence(1..2).fake()
    }
    fn content() -> String {
        Paragraph(1..10).fake()
    }
    fn email() -> Email {
        Email::parse(Secret::new(SafeEmail().fake())).unwrap()
    }
    fn email_client(base_url: String) -> PostmarkEmailClient {
        let http_client = Client::builder()
            .timeout(test::email_client::TIMEOUT)
            .build()
            .unwrap();
        PostmarkEmailClient::new(base_url, email(), Secret::new(Faker.fake()), http_client)
    }

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
                    && body.get("MessageStream").is_some()
            } else {
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists(POSTMARK_AUTH_HEADER))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(&email(), &subject(), &content())
            .await;

        assert!(outcome.is_ok());
    }
    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(&email(), &subject(), &content())
            .await;

        assert!(outcome.is_err());
    }
    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180)); // 3 minutes!
        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(&email(), &subject(), &content())
            .await;

        assert!(outcome.is_err());
    }
}
