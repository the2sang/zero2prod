use crate::domain::SubscriberEmail;
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: SecretString,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: SecretString,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_centent: &str,
        text_centent: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
           from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_centent.to_owned(),
            text_body: text_centent.to_owned(),
        };
        self.http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Sentence, Paragraph};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use secrecy::Secret;
    use wiremock::matchers::{header_exists, header, path, method};
    use wiremock::Request;

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> =
                serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                dbg!(&body);
                body.get("From").is_some()
                && body.get("To").is_some()
                && body.get("Subject").is_some()
                && body.get("HtmlBody").is_some()
                && body.get("TextBody").is_some()
            } else {
                false
            }
        }
    }

    /// Generate a random email subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }

    /// Generate a random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }

    /// Generate a random subscriber email
    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    /// Get a test instance of `EmailClient`.
    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url,
            email(),
            Secret::new(Faker.fake()),
            std::time::Duration::from_millis(200),
        )
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        //Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        //Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        //Assert
    }
}

