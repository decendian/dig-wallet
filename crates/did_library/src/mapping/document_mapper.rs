use crate::did::core::did_document::{DIDDocument, Authentication, VerificationMethod, Service};
use crate::dto::document::DIDDocumentDTO;

/// Mapper for converting between DID document entity and DTO
pub struct DIDDocumentMapper;

impl DIDDocumentMapper {
    /// Convert a DID document entity to a DTO
    pub fn to_dto(entity: &DIDDocument) -> DIDDocumentDTO {
        DIDDocumentDTO {
            context: entity.context.clone(),
            id: entity.id.clone(),
            key_type: entity.key_type.clone(),
            verification_method: entity.verification_method.iter()
                .map(|vm| Self::map_verification_method_to_dto(vm))
                .collect(),
            authentication: entity.authentication.iter()
                .map(|auth| Self::map_authentication_to_dto(auth))
                .collect(),
            assertion_method: entity.assertion_method.clone(),
            key_agreement: entity.key_agreement.clone(),
            capability_invocation: entity.capability_invocation.clone(),
            capability_delegation: entity.capability_delegation.clone(),
            service: entity.service.iter()
                .map(|svc| Self::map_service_to_dto(svc))
                .collect(),
        }
    }

    /// Convert a DID document DTO to an entity
    pub fn to_entity(self ,dto: &DIDDocumentDTO) -> DIDDocument {
        let mut document = DIDDocument::new(&dto.id, dto.key_type);
        
        // Convert all verification methods
        let verification_methods: Vec<VerificationMethod> = dto.verification_method.iter()
            .map(|vm_dto| Self::map_verification_method_to_entity(vm_dto))
            .collect();
        document.add_verification_method(&verification_methods);
        
        // Convert all authentication methods
        let authentication_methods: Vec<Authentication> = dto.authentication.iter()
            .map(|auth_dto| Self::map_authentication_to_entity(auth_dto))
            .collect();
        document.add_authentication(&authentication_methods);
        
        // Add remaining fields
        document.add_assertion_method(&dto.assertion_method);
        document.add_key_agreement(&dto.key_agreement);
        document.add_capability_invocation(&dto.capability_invocation);
        document.add_capability_delegation(&dto.capability_delegation);
        
        // Convert all services
        let services: Vec<Service> = dto.service.iter()
            .map(|svc_dto| Self::map_service_to_entity(svc_dto))
            .collect();
        document.add_service(&services);
        
        document
    }
    
    // Helper method to map VerificationMethod to DTO
    fn map_verification_method_to_dto(vm: &VerificationMethod) -> crate::dto::document::VerificationMethodDTO {
        crate::dto::document::VerificationMethodDTO {
            id: vm.id.clone(),
            vm_type: vm.vm_type.clone(),
            controller: vm.controller.clone(),
            key_material:  vm.key_material.clone()
          
        }
    }
    
    // Helper method to map VerificationMethodDTO to entity
    fn map_verification_method_to_entity(vm_dto: &crate::dto::document::VerificationMethodDTO) -> VerificationMethod {
        VerificationMethod {
            id: vm_dto.id.clone(),
            vm_type: vm_dto.vm_type.clone(),
            controller: vm_dto.controller.clone(),
            key_material: vm_dto.key_material.clone(),
        }
    }
    
    // Helper method to map Authentication to DTO
    fn map_authentication_to_dto(auth: &Authentication) -> crate::dto::document::AuthenticationDTO {
        match auth {
            Authentication::Reference(reference) => {
                crate::dto::document::AuthenticationDTO::Reference(reference.clone())
            },
            Authentication::Embedded(vm) => {
                crate::dto::document::AuthenticationDTO::Embedded(Self::map_verification_method_to_dto(vm))
            }
        }
    }
    
    // Helper method to map AuthenticationDTO to entity
    fn map_authentication_to_entity(auth_dto: &crate::dto::document::AuthenticationDTO) -> Authentication {
        match auth_dto {
            crate::dto::document::AuthenticationDTO::Reference(reference) => {
                Authentication::Reference(reference.clone())
            },
            crate::dto::document::AuthenticationDTO::Embedded(vm_dto) => {
                Authentication::Embedded(Self::map_verification_method_to_entity(vm_dto))
            }
        }
    }
    
    // Helper method to map Service to DTO
    fn map_service_to_dto(svc: &Service) -> crate::dto::document::ServiceDTO {
        crate::dto::document::ServiceDTO {
            id: svc.id.clone(),
            service_type: svc.service_type.clone(),
            service_endpoint: svc.service_endpoint.clone(),
            properties: svc.properties.clone(),
        }
    }
    
    // Helper method to map ServiceDTO to entity
    fn map_service_to_entity(svc_dto: &crate::dto::document::ServiceDTO) -> Service {
        Service {
            id: svc_dto.id.clone(),
            service_type: svc_dto.service_type.clone(),
            service_endpoint: svc_dto.service_endpoint.clone(),
            properties: svc_dto.properties.clone(),
        }
    }
}