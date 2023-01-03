#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

use rocket::serde::{json::Json, Deserialize};

mod mail;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Email<'r> {
    address: &'r str,
}

#[post("/request-otp", format = "application/json", data = "<email>")]
fn request_otp(email: Json<Email<'_>>) {
    let length: usize = 6;
    let otp: &str = &utils::random_string(length);
    let mut text: String = "Your One-Time Password is ".to_owned();
    let mut html: String = "Your One-Time Password is <b>".to_owned();
    let html_end: &str = "</b>";

    text.push_str(otp);
    html.push_str(otp);
    html.push_str(html_end);

    let mail_data = mail::MailData {
        from_address: String::from("mcwake-dev@outlook.com"),
        to_address: String::from(email.address),
        subject: String::from("Your One-Time Password"),
        text,
        html,
    };

    mail::send_email(mail_data);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![request_otp])
}
