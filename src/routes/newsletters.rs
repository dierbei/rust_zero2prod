use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::routes::{error_chain_fmt, send_confirmation_email};
use actix_http::StatusCode;
use actix_web::HttpResponse;
use actix_web::{web, ResponseError};
use anyhow::Context;
use log::error;
use sqlx::PgPool;
use std::fmt::Formatter;
use actix_web::dev::always_ready;

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

// Same logic to get the full error chain on `Debug`
impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PublishError {
    fn status_code(&self) -> StatusCode {
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

pub async fn publish_newsletter(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, PublishError> {
    let subscribers = get_confirmed_subscribers(&pool).await?;
    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(
                        &subscriber.email,
                        &body.title,
                        &body.content.html,
                        &body.content.text,
                    )
                    .await
                    .with_context(|| {
                        format!(
                            "Failed to send newsletter issue to {}",
                            subscriber.email
                        )
                    })?;
            }
            Err(error) => {
                tracing::warn!(
                    // We record the error chain as a structured field
                    // on the log record.
                    error.cause_chain = ?error,
                    // Using `\` to split a long string literal over
                    // two lines, without creating a `\n` character.
                    "Skipping a confirmed subscriber. \
                    Their stored contact details are invalid",
                );
            }
        }
    }
    Ok(HttpResponse::Ok().finish())
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(name = "Adding a new subscriber", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    // We only need `Row` to map the data coming out of this query.
    // Nesting its definition inside the function itself is a simple way
    // to clearly communicate this coupling (and to ensure it doesn't
    // get used elsewhere by mistake).
    // struct Row {
    //     email: String,
    // }

    // let rows = sqlx::query_as!(
    //     Row,
    //     r#"
    //     SELECT email
    //     FROM subscriptions
    //     WHERE status = 'confirmed'
    //     "#,
    // )
    // .fetch_all(pool)
    // .await?;
    //
    // let confirmed_subscribers = rows
    //     .into_iter()
    //     // No longer using `filter_map`!
    //     .map(|r| match SubscriberEmail::parse(r.email) {
    //         Ok(email) => Ok(ConfirmedSubscriber { email }),
    //         Err(error) => Err(anyhow::anyhow!(error)),
    //     })
    //     .collect();

    let confirmed_subscribers = sqlx::query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#,
    )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| match SubscriberEmail::parse(r.email) {
            Ok(email) => Ok(ConfirmedSubscriber { email }),
            Err(error) => Err(anyhow::anyhow!(error)),
        })
        .collect();

    // let confirmed_subscribers = rows
    //     .into_iter()
    //     .filter_map(|r| match SubscriberEmail::parse(r.email) {
    //         Ok(email) => Some(ConfirmedSubscriber { email }),
    //         Err(error) => {
    //             tracing::warn!(
    //                 "A confirmed subscriber is using an invalid email address.\n{}.",
    //                 error
    //             );
    //             None
    //         }
    //     })
    //     .collect();

    Ok(confirmed_subscribers)
}
