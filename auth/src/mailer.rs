use anyhow::{Ok, Result};
use kafka::producer::{Producer, Record, RequiredAcks, DEFAULT_ACK_TIMEOUT_MILLIS};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;
use std::time::Duration;

use crate::constants::EMAIL_QUEUE;
/// the mailer is a tiny layer over Apache Kafka
/// it allows a pub sub communication between the auth service and the email service  in that it receives email payload and adds it to a message broker
/// the email service on the other end takes the email, feeds the data to a templates than sends it to the user
///

#[derive(Debug, Serialize, Deserialize)]
pub struct Mailer<T> {
    pub email: String,
    pub subject: String,
    pub template: String,
    pub data: T,
}

#[derive(Debug)]
pub enum EmailTemplate {
    Signup,
    Welcome,
    PasswordReset,
    ForgottenPassword,
}

// for each of the email template, return a tuple of string, containing the template and the subject name
impl EmailTemplate {
    pub fn get_template(&self) -> (&str, &str) {
        match self {
            EmailTemplate::Signup => ("sign-up", "Account Activation"),
            EmailTemplate::Welcome => ("welcome", "Welcome to Martus"),
            EmailTemplate::PasswordReset => ("password-update", "Martus Password Reset"),
            EmailTemplate::ForgottenPassword => ("forgotten-password", "Password Reset"),
        }
    }
}

/// the email builder enforces correct positional argument for the mailer constructor
pub struct MailBuilder<'a>(pub &'a str, pub EmailTemplate);

impl<'a, T: Serialize + Debug + Deserialize<'a>> Mailer<T> {
    pub fn new(email: &'a str, template: EmailTemplate, data: T) -> Self {
        let (email_template, email_subject) = template.get_template();

        let mailer = Mailer {
            data,
            email: email.to_string(),
            subject: email_subject.to_string(),
            template: email_template.to_string(),
        };
        mailer
    }

    pub async fn send(&self) -> Result<()> {
        let timeout = DEFAULT_ACK_TIMEOUT_MILLIS;
        let acks = RequiredAcks::One;
        let kafka_host = env::var("KAFKA_HOST")?;
        let payload = serde_json::to_string(self).unwrap();
        let mut producer = Producer::from_hosts(vec![kafka_host.to_owned()])
            .with_ack_timeout(Duration::from_millis(timeout))
            .with_required_acks(acks)
            .create()
            .unwrap();
        let record = Record::from_value(EMAIL_QUEUE, payload.as_bytes());
        // add email to the queue
        let status = producer.send(&record).ok();
        match status {
            Some(_) => println!("email sent"),
            _ => println!("error sending email: ",),
        }
        Ok(())
    }
}
