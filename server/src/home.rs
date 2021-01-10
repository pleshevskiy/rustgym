use super::app_data::AppData;
use super::context::*;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

#[get("/")]
pub async fn home(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    HomeContext::new(AppContext::new(data), req.path().to_string()).render_wrapper()
}
