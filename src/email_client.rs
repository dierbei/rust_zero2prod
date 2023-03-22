use crate::domain::SubscriberEmail;
use reqwest::Client;
use tracing_subscriber::fmt::format;

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        // You can do better using `reqwest::Url::join` if you change
        // `base_url`'s type from `String` to `reqwest::Url`.
        // I'll leave it as an exercise for the reader!
        // let base_url = reqwest::Url::parse(&self.base_url).unwrap();
        // let new_url = base_url.join("/email").unwrap();

        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned(),
        };
        let builder = self.http_client.post(&url).json(&request_body);
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use tracing::Instrument;
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // request
        //     curl "https://api.postmarkapp.com/email" \
        //     -X POST \
        //     -H "Accept: application/json" \
        //     -H "Content-Type: application/json" \
        //     -H "X-Postmark-Server-Token: server token" \
        //     -d '{
        //     "From": "sender@example.com",
        //     "To": "receiver@example.com",
        //     "Subject": "Postmark test",
        //     "TextBody": "Hello dear Postmark user.",
        //     "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>"
        // }'
        // response
        // {
        //     "To": "receiver@example.com",
        //     "SubmittedAt": "2021-01-12T07:25:01.4178645-05:00",
        //     "MessageID": "0a129aee-e1cd-480d-b08d-4f48548ff48d",
        //     "ErrorCode": 0,
        //     "Message": "OK"
        // }

        // Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender);

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client.send_email(subscriber_email, &subject, &content, &content);

        // Assert
    }
}