use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct MailData {
    pub from_address: String,
    pub to_address: String,
    pub subject: String,
    pub html: String,
    pub text: String,
}

pub fn send_email(
    mail_data: MailData,
) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
    dotenv().ok();
    let email = Message::builder()
        .from(mail_data.from_address.parse().unwrap())
        .to(mail_data.to_address.parse().unwrap())
        .subject(mail_data.subject)
        .body(mail_data.html)
        .unwrap();
    let sg_api_key = dotenv_codegen::dotenv!("SENDGRID_API_KEY");
    let credentials = Credentials::new("apikey".to_string(), sg_api_key.to_string());
    let mailer = SmtpTransport::relay("smtp.sendgrid.net")
        .unwrap()
        .credentials(credentials)
        .build();

    return mailer.send(&email);
}
