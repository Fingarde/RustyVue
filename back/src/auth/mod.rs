use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let token = credentials.token();

    if token == "123" {
        return Ok(req);
    }

    Err((
        actix_web::error::ErrorUnauthorized("Invalid token").into(),
        req,
    ))
}
