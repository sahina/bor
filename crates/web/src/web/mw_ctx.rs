use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::mw_auth::parse_token;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_owned());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // todo: token component validations
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // remove cookie if something went wrong
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    // store cxt_result in request extension
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}
