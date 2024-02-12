use cslt::{login::LoginBySecret, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;
    let now = Instant::now();
    {
        let client = Client::new();

        let login_by_secret =
            LoginBySecret::new("43756F61692F4739743362774C6371724971356955494A4C6B524B6A45494B7A");

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
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
