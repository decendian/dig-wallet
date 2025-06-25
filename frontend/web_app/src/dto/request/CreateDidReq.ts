export interface CreateDidRequestDTO {
    method: string;
    keyType?: string;
    options?: Record<string, any>;
}

/**
 * Creates a new CreateDidRequestDTO object with the specified parameters
 * @param method The DID method to use (e.g., 'ethr', 'key', 'web')
 * @param keyType Optional key type (e.g., 'Ed25519', 'Secp256k1', 'P256')
 * @param options Optional additional parameters for DID creation
 * @returns A CreateDidRequestDTO object
 */
export function createDidRequest(
    method: string,
    keyType?: string,
    options?: Record<string, any>
): CreateDidRequestDTO {
    return {
        method,
        keyType,
        options
    };
}

/**
 * Validates a CreateDidRequestDTO object
 * @param request The DTO to validate
 * @returns True if valid, false otherwise
 */
export function validateDidRequest(request: CreateDidRequestDTO): boolean {
    return !(!request.method || request.method.trim() === '');
}

/**
 * Converts a CreateDidRequestDTO to a JSON string
 * @param request The DTO to convert
 * @returns A JSON string representation of the DTO
 */
export function serializeDidRequest(request: CreateDidRequestDTO): string {
    return JSON.stringify(request);
}

/**
 * Creates a CreateDidRequestDTO from a JSON string
 * @param json The JSON string to parse
 * @returns A CreateDidRequestDTO object
 */
export function deserializeDidRequest(json: string): CreateDidRequestDTO {
    return JSON.parse(json) as CreateDidRequestDTO;
}