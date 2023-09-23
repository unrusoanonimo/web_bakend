#![allow(unreachable_code)]

use std::{
    error::Error,
    sync::{Mutex, MutexGuard},
};

use config::Config;

use once_cell::sync::Lazy;
use rouille::{Request, Response};
use router::api::api_router;
use utils::{create_db, Modules};

mod config;
mod data;
mod model;
mod router;
mod utils;

use crate::{
    router::public_dir,
    utils::{base_ur, empty_response_from_esstatus_code},
};
type AppResult<T> = Result<T, Box<dyn Error>>;

static CONFIG: Lazy<Config> = Lazy::new(|| Config::open("config.json").unwrap());
fn main() -> Result<(), Box<dyn Error>> {
    let mut conection = model::DbConections::UraLoader.open()?;

    create_db(&mut conection)?;
    let modules = Modules::new(conection);

    let mutex_modules = Mutex::new(modules);
    println!("HTTP server on 127.0.0.1:{}", CONFIG.port);
    rouille::start_server(format!("0.0.0.0:{}", CONFIG.port), move |request| {
        let mut modules: MutexGuard<'_, Modules> = mutex_modules.lock().unwrap();
        modules.fill_none();

        response_randler(request, modules).unwrap_or(empty_response_from_esstatus_code(500))
    });
    unreachable!();
}
const API_PATH: &str = "/api";
fn response_randler(req: &Request, modules: MutexGuard<'_, Modules>) -> AppResult<Response> {
    let url: &str = &base_ur(req.raw_url());

    match (req.method(), url) {
        // ("GET", ROOT) => Ok(Response::html(r#"<a href="./login">link</a>"#)),
        _ if url.starts_with(API_PATH) => api_router(req, modules, &url[API_PATH.len()..]),
        ("GET", _) => public_dir(req, modules, url),
        _ => Ok(Response::empty_404()),
    }
}
