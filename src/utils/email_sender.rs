use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

pub fn send_email(to: String, content: String) {
    let smtp_address = "smtp.gmail.com";
    let username = "your_email";
    let password = "your_password";

    let email = EmailBuilder::new()
        .to(to)
        .from(username)
        .subject("Welcome!")
        .html(content)
        .build()
        .unwrap()
        .into();
    let credentials = (username, password).into_credentials();
    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();
    let result = client.send(email);
    if !result.is_ok() {
        eprintln!("err {:?}", result.err().unwrap());
    } else {
        println!("email sent {:?}", result);
    }
}
