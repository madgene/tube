#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate shell;
extern crate config;
#[macro_use] extern crate lazy_static;

mod settings;

use rocket::request::{ Form };
use rocket::response::Redirect;
use rocket_contrib::static_files::{ StaticFiles, Options };
use rocket_contrib::Template;

#[derive(Serialize)]
struct File {
    name: String,
    url: String
}

#[derive(Serialize)]
struct TemplateContext {
    files: Vec<File>
}

#[derive(FromForm)]
struct Link {
    url: String
}

#[get("/")]
fn index() -> Template {
    let path: &String = &settings::get().files.path;
    let output: String = cmd!(&format!("ls {}", path)).stdout_utf8().unwrap();
    let names: Vec<&str> = output.split("\n").collect();
    let files: Vec<File> = names
        .iter()
        .filter(|name| name.len() > 0)
        .map(|name| {
            let url = format!("/files/{}", name);
            File { name: name.to_string(), url: url }
        })
    .collect();
    let context = TemplateContext { files: files };
    Template::render("index", &context)
}

#[post("/", data = "<link>")]
fn download(link: Form<Link>) -> Redirect {
    let path: &String = &settings::get().files.path;
    cmd!(&format!("youtube-dl -o {}/%(title)s.%(ext)s {}", path, link.url)).spawn().unwrap();
    Redirect::to("/")
}

fn main() {
    let dir: &String = &settings::get().files.path;
    let options = Options::Index | Options::DotFiles;
    rocket::ignite()
        .mount("/", routes![index, download])
        .mount("/files", StaticFiles::new(dir, options))
        .attach(Template::fairing())
        .launch();
}
