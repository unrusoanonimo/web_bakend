use std::sync::MutexGuard;

use rouille::{Request, Response};

use crate::{
    utils::{AppError, Modules},
    AppResult,
};

use super::ROOT;

pub fn api_router(
    req: &Request,
    modules: MutexGuard<'_, Modules>,
    url: &str,
) -> AppResult<Response> {
    match (req.method(), url) {
        ("GET", ROOT) => Ok(Response::text("text")),
        ("GET", "/t") => Ok(Response::text("asd")),
        ("GET", "/user") => get_user_by_id(req, modules),
        _ => Ok(Response::empty_400()),
    }
}

fn get_user_by_id(req: &Request, mut modules: MutexGuard<'_, Modules>) -> AppResult<Response> {
    let token = req.get_param("token").ok_or(AppError::UnexpectedNone)?;
    let user = modules.data_access().user_dao.get_by_token(&token)?;
    Ok(Response::json(&user))
}
