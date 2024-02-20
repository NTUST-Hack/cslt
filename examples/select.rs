use std::env;

use cslt::{client::SelectMode, login::LoginBySecret, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = env::var("NTUSTSECRET").expect("$NTUSTSECRET is not set");

    use std::time::Instant;
    let now = Instant::now();
    {
        let client = Client::new();

        let login_by_secret = LoginBySecret::new(&secret);

        client.login(&login_by_secret).await?;

        let details = client.refresh_details().await?;
        let name = details.name()?;
        let class = details.class()?;

        println!("logined?: {}", details.is_logined());
        println!("name: {name}");
        println!("class: {class}");

        let courses = details.courses()?;

        for c in courses {
            println!(
                "{: <10} | {:　<10} | {: <10} | {: <10} | {: <10} | {: <10}",
                c.course_no, c.name, c.credits, c.required, c.teacher, c.notes
            );
        }

        let select_result = client
            .select_course(SelectMode::Started, "AT2003301")
            .await?
            .result()
            .await?;

        let result_message = select_result
            .result_message()
            .unwrap_or("no message".to_string());

        println!("result message: {}", result_message);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
