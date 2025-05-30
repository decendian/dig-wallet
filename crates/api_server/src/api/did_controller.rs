use actix_web::{web, HttpResponse, Responder};
use shared::dto::DataTransferObject;
use crate::dto::request::CreateDidRequestDTO;
use crate::dto::response::{ApiResponseDTO, CreateDidResponseDTO, ErrorDTO};

pub async fn create_did(
  data: web::Json<CreateDidRequestDTO>
) -> impl Responder {
  // Validate the DTO
  if let Err(validation_error) = data.validate() {
    return HttpResponse::BadRequest().json(
      ApiResponseDTO::<CreateDidResponseDTO> {
        success: false,
        data: None,
        error: Some(ErrorDTO {
          code: "VALIDATION_ERROR".to_string(),
          message: validation_error,
          details: None,
        }),
    });
  }

  // Implement the DID creation logic directly here instead of calling service.create_did
  // For example:
  let result = create_did_implementation(&data).await;
  
  match result {
    Ok(result) => {
      HttpResponse::Created().json(ApiResponseDTO {
        success: true,
        data: Some(result),
        error: None,
      })
    },
    Err(e) => {
      HttpResponse::InternalServerError().json(ApiResponseDTO::<CreateDidResponseDTO>  {
        success: false,
        data: None,
        error: Some(ErrorDTO {
          code: "INTERNAL_ERROR".to_string(),
          message: e.to_string(),
          details: None,
        }),
      })
    }
  }
}

// Helper function to implement DID creation logic
async fn create_did_implementation(data: &CreateDidRequestDTO) -> Result<CreateDidResponseDTO, String> {
    // Implement your DID creation logic here
    // ...
  todo!()
}