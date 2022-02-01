use tide::{Response, Request, Body, Server};
use async_std::sync::RwLock;
//use tide::prelude::{Deserialize, Serialize};
use std::collections::hash_map::{HashMap};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
// use serde::{Deserialize, Serialize};

#[derive(Clone,Debug)]
struct State {
    jargons: Arc<RwLock<HashMap<String,Jargon>>>
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
struct Jargon {
    name: String,
    def: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Jargons {
    jargon: Vec<Jargon>,
}

#[async_std::main]
async fn main() {
    tide::log::start();
    let jargon_store = Default::default() ;
    let app = server(jargon_store).await;

    app.listen("127.0.0.1:8080").await.unwrap();
}

async fn server(jargon_store: Arc <RwLock<HashMap<String, Jargon>>>) -> Server<State> {
    let state = State {
        jargons: jargon_store, //Default::default(),
    };

    let mut app = tide::with_state(state);

    app.at("/").get(|_| async { Ok("Hello, world!") });

    app.at("/jargon").post(|mut req: Request<State>| async move {
        let jargon: Jargon = req.body_json().await?;
        let mut jargons = req.state().jargons.write().await;
        jargons.insert(String::from(&jargon.name), jargon.clone());
        let mut res = Response::new(201);
        res.set_body(Body::from_json(&jargon)?);
        Ok(res)
    });

    app.at("/jargon").get(|req: Request<State>| async move {
        let jargons = req.state().jargons.read().await;
        let jargon_vec : Vec<Jargon> = jargons.values().cloned().collect(); 
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&jargon_vec)?);
        Ok(res)

    });
    app
}

#[async_std::test]
async fn add_jargon() -> tide::Result<()>{
    use tide::http::{Method, Request, Response, Url};

    let should_be = "{\"name\":\"foo\",\"def\":\"An arbitrary name for something to avoid taking focus away from the point\"}";

    let jargon = Jargon {
         name : String::from("foo"),
         def : String::from("An arbitrary name for something to avoid taking focus away from the point"),
    };

    let mut jargon_store = HashMap::new();
    //jargon_store.insert(jargon.name.clone(), jargon);

    let state = Arc::new(RwLock::new(jargon_store));
    let app = server(state).await;

    let url = Url::parse("https://example.com/jargon").unwrap();
    let mut req = Request::new(Method::Post, url);
    req.set_body(serde_json::to_string(&jargon)?);

    let mut res: Response = app.respond(req).await.unwrap();
    let was = res.body_string().await.unwrap();

    assert_eq!(should_be, was);
    Ok(())
}

#[async_std::test]
async fn list_jargons() -> tide::Result<()>{
    use tide::http::{Method, Request, Response, Url};

    let should_be = "[{\"name\":\"foo\",\"def\":\"An arbitrary name for something to avoid taking focus away from the point\"}]";

    let jargon = Jargon {
         name : String::from("foo"),
         def : String::from("An arbitrary name for something to avoid taking focus away from the point"),
    };

    let mut jargon_store = HashMap::new();
    jargon_store.insert(jargon.name.clone(), jargon);

    let state = Arc::new(RwLock::new(jargon_store));
    let app = server(state).await;

    let url = Url::parse("https://example.com/jargon").unwrap();
    let req = Request::new(Method::Get, url);

    let mut res: Response = app.respond(req).await.unwrap();
    let was = res.body_string().await.unwrap();

    assert_eq!(should_be, was);
    Ok(())
}