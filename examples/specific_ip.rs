use std::{env, net::IpAddr};

use cslt::{client::ClientBuilder, login::LoginBySecret};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = env::var("NTUSTSECRET").expect("$NTUSTSECRET is not set");

    let client = ClientBuilder::new()
        .local_address(IpAddr::V4("122.118.194.21".parse()?))
        .build()?;

    let login_by_secret = LoginBySecret::new(&secret);

    client.login(&login_by_secret).await?;

    let details = client.refresh_details().await?;

    use std::time::Instant;
    let now = Instant::now();
    {
        let logined = details.is_logined();
        println!("logined?: {}", logined);

        if logined {
            let name = details.name()?;
            let class = details.class()?;

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
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
