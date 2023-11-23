use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

pub fn send_mail(to: &String, content: String) {
    let letter = Message::builder()
        .from("498315775@qq.com".parse().unwrap())
        .reply_to("498315776@qq.com".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Task Infomation".to_owned())
        .header(ContentType::TEXT_HTML)
        .body(content)
        .unwrap();

    let creds = Credentials::new("498315776@qq.com".to_owned(), "oajfodas".to_owned());

    let mailer = SmtpTransport::relay("smtp.qq.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&letter) {
        Ok(_)=>{},
        Err(e)=>{}
    }
}
