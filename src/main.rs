#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;

use rocket::tokio::time::{sleep, Duration};

use rocket::Request;

use rocket::response::status;

use rocket_dyn_templates::{Template, context};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/hello/<name>")]
fn terahello(name :&str) -> Template {
    Template::render("tera/hello", context! {
        name: Some(name)
    })
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"See <br/><a href="tera">Tera</a><br/><a href="hbs">Handlebars</a>."#)
}

#[get("/")]
fn teraindex() -> Template {
    Template::render("tera/index", context! {
        title: "Hello",
        name: Some("hugo"),
        items: vec!["One", "Two", "Three"],
    })
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Désolé :)\n'{}' n'est pas un chemin valide.", req.uri())
}


#[get("/<id>")]
fn new(id: usize) -> status::Accepted<String> {
    status::Accepted(Some(format!("id: '{}'", id)))
}

#[rocket::main]
async fn main() -> Result<(),rocket::Error> {
    let _rocket = rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index,delay,new])
        .mount("/tera", routes![teraindex,terahello])        
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
}
