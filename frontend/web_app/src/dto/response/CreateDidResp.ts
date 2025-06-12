export interface VerificationMethod {
    id: string;
    vm_type: string;
    controller: string;
    key_material: KeyMaterial;
}

export type Authentication = string | VerificationMethod;

export interface Service {
    id: string;
    service_type: string;
    service_endpoint?: string;
    properties: Record<string, any>;
}

export type KeyMaterial = 
    | { public_key_jwk: any }
    | { public_key_multibase: string }
    | { public_key_hex: string };

export interface CreateDidResponseDTO {
    context: string[];
    id: string;
    key_type: string;
    verification_method: VerificationMethod[];
    authentication: Authentication[];
    assertion_method: string[];
    key_agreement: string[];
    capability_invocation: string[];
    capability_delegation: string[];
    service: Service[];
}

/**
 * Validates a CreateDidResponseDTO object
 * @param response The DTO to validate
 * @returns True if valid, false otherwise
 */
export function validateDidResponse(response: CreateDidResponseDTO): boolean {
    if (!response.id || response.id.trim() === '') {
        return false;
    }
    if (!response.context || response.context.length === 0) {
        return false;
    }
    return true;
}

/**
 * Parses a JSON string into a CreateDidResponseDTO object
 * @param json The JSON string to parse
 * @returns A CreateDidResponseDTO object
 */
export function deserializeDidResponse(json: string): CreateDidResponseDTO {
    return JSON.parse(json) as CreateDidResponseDTO;
}

/**
 * Extracts just the DID string from the full DID document
 * @param response The full DID document response
 * @returns The DID string (e.g., "did:method:identifier")
 */
export function getDidString(response: CreateDidResponseDTO): string {
    return response.id;
}