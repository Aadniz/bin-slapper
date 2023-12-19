mod context;
mod utils;
mod hbs;
mod error_pages;

use rocket_dyn_templates::Template;
use rocket::fs::FileServer;


#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_include_static_resources;

// Make the favicon appear to be served from root
static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
}

#[get("/")]
fn index() -> Template {
    let cxt = context::Sorts::new("Rocket");
    Template::render("index", &cxt)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::custom(|engines| hbs::register_helpers(&mut engines.handlebars)))
        .attach(static_resources_initializer!(
            "favicon" => "static/img/favicon.ico"
        ))
        .mount("/", routes![favicon])
        .mount("/", routes![index])
        .register("/", catchers![error_pages::default_catcher])

        // Serve the static files
        .mount("/static", FileServer::from("static"))
}