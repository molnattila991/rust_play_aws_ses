#![allow(unused)]

use std::{env, path::Path};

use anyhow::{bail, Context};
use aws_sdk_sesv2::{
    config,
    model::{Body, Content, Destination, EmailContent, Message},
    Client, Credentials, Error, Region,
};
use dotenv::dotenv;

const BUCKET_NAME: &str = "rate-n-date-profile-images";

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    send_message(
        &client,
        "molnarattila991@gmail.com",
        "kiazmitakar@gmail.com",
        "subject",
        "message",
    )
    .await;

    Ok(())
}

async fn send_message(
    client: &Client,
    list: &str,
    from: &str,
    subject: &str,
    message: &str,
) -> Result<(), Error> {
    let dest = Destination::builder().to_addresses(list).build();
    let subject_content = Content::builder().data(subject).charset("UTF-8").build();
    let body_content = Content::builder().data(message).charset("UTF-8").build();
    let body = Body::builder().text(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    client
        .send_email()
        .from_email_address(from)
        .destination(dest)
        .content(email_content)
        .send()
        .await
        .unwrap();

    println!("Email sent to list");

    Ok(())
}
