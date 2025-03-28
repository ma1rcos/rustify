use serde::{Deserialize, Serialize};
use actix_web::HttpResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    message: String,
    data: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    status_code: u16,
    message: String,
    data: Option<String>,
}

impl Response {

    pub fn get_success(message: String, data: String) -> HttpResponse {
        HttpResponse::Accepted().json(ResponseBody {
            status_code: 200,
            message,
            data: Some(data),
        })
    }

    pub fn get_created(message: String, data: String) -> HttpResponse {
        HttpResponse::Created().json(ResponseBody {
            status_code: 201,
            message,
            data: Some(data),
        })
    }

    pub fn get_forbidden(message: String) -> HttpResponse {
        HttpResponse::Forbidden().json(ResponseBody {
            status_code: 403,
            message,
            data: None,
        })
    }

    pub fn get_unauthorized(message: String) -> HttpResponse {
        HttpResponse::NotFound().json(ResponseBody {
            status_code: 404,
            message,
            data: None,
        })
    }

    pub fn get_not_found(message: String) -> HttpResponse {
        HttpResponse::NotFound().json(ResponseBody {
            status_code: 404,
            message,
            data: None,
        })
    }

    pub fn get_conflict(message: String) -> HttpResponse {
        HttpResponse::Conflict().json(ResponseBody {
            status_code: 409,
            message,
            data: None,
        })
    }

    pub fn get_internal_server_error(message: String) -> HttpResponse {
        HttpResponse::InternalServerError().json(ResponseBody {
            status_code: 500,
            message,
            data: None,
        })
    }

}