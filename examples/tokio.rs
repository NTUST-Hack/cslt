use std::env;

use cslt::{login::LoginBySecret, Client};

async fn run(ntustsecret: &str) -> anyhow::Result<()> {
    let client = Client::new();

    let login_by_secret = LoginBySecret::new(ntustsecret);

    client.login(&login_by_secret).await?;

    let details = client.refresh_details().await?;
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = env::var("NTUSTSECRET").expect("$NTUSTSECRET is not set");

    tokio::spawn(async move { run(&secret).await })
        .await
        .unwrap()
        .unwrap();

    Ok(())
}
