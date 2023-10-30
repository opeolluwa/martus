use serde::{Deserialize, Serialize};

/// the mailer is a tiny layer over Apache Kafka
/// it allows a pub sub communication between the auth service and the email service  in that it receives email payload and adds it to a message broker
/// the email service on the other end takes the email, feeds the data to a templates than sends it
///

#[derive(Debug, Serialize, Deserialize)]
pub struct Mailer {
    pub email: String,
    pub subject: String,
    pub template: String,
}

#[derive(Debug)]
pub enum EmailTemplate {
    Signup,
    Welcome,
    PasswordReset,
}

// for each of the email template, return a tuple of string, containing the template and the subject name
impl EmailTemplate {
    pub fn get_template(&self) -> (&str, &str) {
        match self {
            EmailTemplate::Signup => ("signup", "Welcome to Martus"),
            EmailTemplate::Welcome => ("welcome", "Welcome to Martus"),
            EmailTemplate::PasswordReset => ("password_reset", "Martus Password Reset"),
        }
    }
}

/// the email builder enforces correct positional argument for the mailer constructor
pub struct MailBuilder<'a>(pub &'a str, pub EmailTemplate);

impl Mailer {
    pub fn new<'a>(email: &'a str, template: EmailTemplate) -> Self {
        let (email_template, email_subject) = template.get_template();

        let mailer = Mailer {
            email: email.to_string(),
            subject: email_subject.to_string(),
            template: email_template.to_string(),
        };
        mailer
    }

    //TODO: add the mail to the queue
    pub async fn send(&self) -> () {
        println!("send email");
    }
}
