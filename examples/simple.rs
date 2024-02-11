use cslt::{login::LoginBySecret, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;
    let now = Instant::now();

    {
        let client = Client::new();

        let login_by_secret =
            LoginBySecret::new("43756F61692F4739743362774C63717249713569554E463571766E38574B394B");

        client.login(&login_by_secret).await?;

        let name = client.refresh_details().await?.name().unwrap();

        println!("{name:?}");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
