use std::env;

use cslt::blocking::{login::LoginBySecret, Client};

fn main() -> anyhow::Result<()> {
    let secret = env::var("NTUSTSECRET").expect("$NTUSTSECRET is not set");

    let client = Client::new();

    let login_by_secret = LoginBySecret::new(&secret);

    client.login(&login_by_secret)?;

    let details = client.refresh_details()?;

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
