extern crate lettre;
extern crate lettre_email;
extern crate mime;

use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};

pub fn send_email(c: &str) -> bool {
    let email_receiver = "lijieliwei@126.com";
    let mine_email = "453220764@qq.com";
    let smtp_server = "smtp.qq.com";
    let password = "grwxnwtqhvikcadg";
    let email = Email::builder()
        .to(email_receiver)
        .from(mine_email)
        .subject("ip changed from my home.")
        .html(format!("# Hi now ip address: {}", c))
        .build()
        .unwrap();

    let creds = Credentials::new(
        mine_email.to_string(),
        password.to_string(),
    );

    // Open connection to qq mail
    let mut mailer = SmtpClient::new_simple(smtp_server)
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

    return if result.is_ok() {
        println!("Email sent success");
        println!("{:?}", result);
        mailer.close();
        true
    } else {
        println!("Could not send email: {:?}", result);
        mailer.close();
        false
    }
}
