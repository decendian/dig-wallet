// web_app/src/dto/did.ts
export interface DIDDocumentDTO {
    '@context': string[];
    id: string;
    keyType: string;
    verificationMethod: VerificationMethodDTO[];
    authentication: (string | VerificationMethodDTO)[];
    assertionMethod: string[];
    keyAgreement: string[];
    capabilityInvocation: string[];
    capabilityDelegation: string[];
    service: ServiceDTO[];
}

export interface VerificationMethodDTO {
    id: string;
    type: string;
    controller: string;
    publicKeyJwk?: any;
    publicKeyMultibase?: string;
    publicKeyHex?: string;
}

export interface ServiceDTO {
    id: string;
    type: string;
    serviceEndpoint?: string;
    [key: string]: any;
}

// Request/response DTOs
export interface CreateDidRequestDTO {
    method: string;
    keyType?: string;
    options?: Record<string, any>;
}

export interface CreateDidResponseDTO {
    did: string;
    document: DIDDocumentDTO;
    keys: Record<string, string>;
}