use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::routes::error_chain_fmt;
// use actix_http::header::HeaderMap;
use actix_web::http::header::{HeaderMap, HeaderValue};
use actix_web::http::{header, StatusCode};
use actix_web::{web, ResponseError};
use actix_web::{HttpRequest, HttpResponse};
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use log::error;
// use sha3::Digest;
use crate::telemetry::spawn_blocking_with_tracing;
use sqlx::PgPool;
use std::fmt::Formatter;

#[derive(thiserror::Error)]
pub enum PublishError {
    // New error variant!
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
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
    fn error_response(&self) -> HttpResponse {
        match self {
            PublishError::UnexpectedError(_) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            PublishError::AuthError(_) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    // actix_web::http::header provides a collection of constants
                    // for the names of several well-known/standard HTTP headers
                    .insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // Return a 401 for auth errors
            PublishError::AuthError(_) => StatusCode::UNAUTHORIZED,
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

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip(body, pool, email_client, request),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn publish_newsletter(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    // New extractor!
    request: HttpRequest,
) -> Result<HttpResponse, PublishError> {
    let credentials = basic_authentication(request.headers())
        // Bubble up the error, performing the necessary conversion
        .map_err(PublishError::AuthError)?;

    let user_id = validate_credentials(credentials, &pool).await?;
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

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
                        format!("Failed to send newsletter issue to {}", subscriber.email)
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

struct Credentials {
    username: String,
    password: String,
}

fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
    // The header value, if present, must be a valid UTF8 string
    let header_value = headers
        .get("Authorization")
        .context("The 'Authorization' header was missing")?
        .to_str()
        .context("The 'Authorization' header was not a valid UTF8 string.")?;

    let base64encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization scheme was not 'Basic'.")?;

    let decoded_bytes = base64::decode_config(base64encoded_segment, base64::STANDARD)
        .context("Failed to base64-decode 'Basic' credentials.")?;

    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF8.")?;

    // Split into two segments, using ':' as delimitator
    let mut credentials = decoded_credentials.splitn(2, ':');

    let username = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth."))?
        .to_string();

    let password = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth."))?
        .to_string();

    Ok(Credentials { username, password })
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, PublishError> {
    // let row: Option<_> = sqlx::query!(
    //     r#"
    //     SELECT user_id, password_hash
    //     FROM users
    //     WHERE username = $1
    //     "#,
    //     credentials.username,
    // )
    // .fetch_optional(pool)
    // .await
    // .context("Failed to perform a query to retrieve stored credentials.")
    // .map_err(PublishError::UnexpectedError)?;

    let mut user_id = None;
    let mut expected_password_hash = "$argon2id$v=19$m=15000,t=2,p=1$\
    gZiV/M1gPc22ElAH/Jh1Hw$\
    CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
        .to_string();

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, pool)
            .await
            .map_err(PublishError::UnexpectedError)?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    // let (user_id, expected_password_hash) = get_stored_credentials(&credentials.username, pool)
    //     .await
    //     .map_err(PublishError::UnexpectedError)?
    //     .ok_or_else(|| PublishError::AuthError(anyhow::anyhow!("Unknown username.")))?;

    // This executes before spawning the new thread
    // let current_span = tracing::Span::current();
    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
        // verify_password_hash(expected_password_hash, credentials.password)
    })
    // actix_web::rt::task::spawn_blocking(move || {
    //     // We then pass ownership to it into the closure
    //     // and explicitly executes all our computation
    //     // within its scope.
    //     current_span.in_scope(|| {
    //         verify_password_hash(expected_password_hash, credentials.password)
    //     })
    // })
    .await
    .context("Failed to spawn blocking task.")
    .map_err(PublishError::UnexpectedError)??;

    // let expected_password_hash = PasswordHash::new(&expected_password_hash)
    //     .context("Failed to parse hash in PHC string format.")
    //     .map_err(PublishError::UnexpectedError)?;
    //
    // tracing::info_span!("Verify password hash")
    //     .in_scope(|| {
    //         Argon2::default()
    //             .verify_password(
    //                 credentials.password.as_bytes(),
    //                 &expected_password_hash
    //             ) })
    //     .context("Invalid password.")
    //     .map_err(PublishError::AuthError)?;

    // Argon2::default()
    //     .verify_password(credentials.password.as_bytes(), &expected_password_hash)
    //     .context("Invalid password.")
    //     .map_err(PublishError::AuthError)?;

    // This is only set to `Some` if we found credentials in the store
    // So, even if the default password ends up matching (somehow)
    // with the provided password,
    // we never authenticate a non-existing user.
    // You can easily add a unit test for that precise scenario.

    user_id.ok_or_else(|| PublishError::AuthError(anyhow::anyhow!("Unknown username.")))
}

// We extracted the db-querying logic in its own function with its own span.
#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(uuid::Uuid, String)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT user_id, password_hash
        FROM users
        WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to retrieve stored credentials.")?
    .map(|row| (row.user_id, row.password_hash));

    Ok(row)
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), PublishError> {
    let expected_password_hash = PasswordHash::new(&expected_password_hash)
        .context("Failed to parse hash in PHC string format.")
        .map_err(PublishError::UnexpectedError)?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .context("Invalid password.")
        .map_err(PublishError::AuthError)
}
