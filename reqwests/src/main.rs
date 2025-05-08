use std::time::Duration;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let url = "https://wikidocs.net/book/16747";
    // let res = reqwest::get("https://wikidocs.net/book/16747")
    //     .await
    //     .unwrap();

    // match res.status() {
    //     reqwest::StatusCode::OK => match res.text().await {
    //         Ok(body) => println!("{}", body),
    //         Err(_) => println!("Error to get text"),
    //     },
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         println!("unauthorized.")
    //     }
    //     other => {
    //         panic!("[panic: {:?}]", other)
    //     }
    // }

    // let body = reqwest::get("https://wikidocs.net/book/16747")
    //     .await?
    //     .text()
    //     .await?;
    // println!("{body}");
    //
    // let client = reqwest::Client::new();
    // let res = client
    //     .get(url)
    //     .header(reqwest::header::CONTENT_TYPE, "application/json")
    //     .header(reqwest::header::ACCEPT, "application/json")
    //     .send()
    //     .await
    //     .unwrap();
    // let client = reqwest::Client::builder()
    //     .timeout(Duration::from_secs(10))
    //     .build()
    //     .unwrap();
    // let req = client
    //     .get(url)
    //     .timeout(Duration::from_secs(60 * 30))
    //     .send()
    //     .await;
    //
    // let req = match req {
    //     Ok(ok) => ok,
    //     Err(err) => {
    //         if err.is_timeout() {
    //             println!("encountered timeout");
    //             return;
    //         } else {
    //             panic!("{err}")
    //         }
    //     }
    // };

    // let bad_url = "https://yyyy.wikidocs.net/book/16747";
    // match retry_send(bad_url).await {
    //     Ok(txt) => println!("Res={}", txt),
    //     Err(err) => eprintln!("Faild to read data:{}", err),
    // }

    // let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
    // println!("body = {body:?}");

    // let client = reqwest::blocking::Client::builder()
    //     .timeout(Duration::from_secs(10))
    //     .build()?;
    // let resp = client.get("http://httpbin.org/").send()?;
    // let body = resp.text()?;
    // println!("body={body:?}");
    //

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?
        .text()
        .await?;

    let params = [("foo", "bar"), ("baz", "quux")];
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let mut map = std::collections::HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    println!("res={}", res);
    Ok(())
}

async fn retry_send(url: &str) -> Result<String, reqwest::Error> {
    let mut cnt = 0;
    let max_cnt = 5;
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    loop {
        let res = client.get(url).send().await;

        match res {
            Ok(parsed) => return Ok(parsed.text().await?),
            Err(err) => {
                cnt += 1;
                if cnt < max_cnt {
                    println!("timeout: retry count = {}", cnt);
                    continue;
                } else {
                    return Err(err);
                }
            }
        };
    }
}
