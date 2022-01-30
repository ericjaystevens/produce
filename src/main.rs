use tide::{Response, Request, Body};
use tide::prelude::{Deserialize, Serialize};
// use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Jargon {
    name: String,
    def: String,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Hello, world!") });

    app.at("/jargon").post(|mut req: Request<()>| async move {
        let jargon: Jargon = req.body_json().await?;
        println!("{:?}", jargon);
        let mut res = Response::new(201);
        res.set_body(Body::from_json(&jargon)?);
        Ok(res)
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}