use std::{fs::File, path::PathBuf, sync::MutexGuard};

use rouille::{Request, Response};

use crate::{
    utils::{get_extension, Modules},
    AppResult,
};
pub mod api;
pub const ROOT: &str = "";
#[allow(dead_code)]
pub fn fake_router(
    _req: &Request,
    _modules: MutexGuard<'_, Modules>,
    _url: &str,
) -> AppResult<Response> {
    panic!();
}

pub fn public_dir(
    req: &Request,
    modules: MutexGuard<'_, Modules>,
    url: &str,
) -> AppResult<Response> {
    let mut inner_path = "./public".to_string();
    inner_path += url;
    let mut path = PathBuf::from(&inner_path);
    if path.is_dir() {
        if !req.raw_url().ends_with('/') {
            let mut new = url.to_owned();
            new.push('/');
            return Ok(Response::redirect_308(new));
        }
        path.push("index.html");
        let file = File::open(path)?;
        return Ok(Response::from_file("text/html", file));
    }
    let file = File::open(path)?;
    let extension = get_extension(&inner_path).unwrap_or_default();
    let mime = modules.mime_form_extension(extension);
    let res = Response::from_file(mime.to_owned(), file);
    Ok(res)
}
