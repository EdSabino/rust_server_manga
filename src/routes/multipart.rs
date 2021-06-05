use rocket_contrib::json::Json;
use rocket::{Data, Route};
use rocket::response::status::Custom;
use rocket::http::{ContentType, Status};

use crate::domain::usecases::images::ReceiveImageUseCase;
use crate::domain::entities::ImageUuid;

#[post("/", data = "<data>")]
fn multipart_upload(cont_type: &ContentType, data: Data) -> Result<Json<ImageUuid>, Custom<String>> {
    if !cont_type.is_form_data() {
        return Err(Custom(
            Status::BadRequest,
            "Content-Type not multipart/form-data".into()
        ));
    }

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
            || Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into()
            )
        )?;
    
    match ReceiveImageUseCase::new().call(data, boundary) {
        Ok(resp) => Ok(Json(ImageUuid::new(resp))),
        Err(err) => Err(Custom(Status::BadRequest, err.to_string()))
    }
}

pub fn create_routes() -> Vec<Route> {
    routes![multipart_upload]
}