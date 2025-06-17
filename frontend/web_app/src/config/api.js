// Get the API URL from environment variables
// CRA automatically injects environment variables that start with REACT_APP_
const API_URL = process.env.REACT_APP_API_URL;

// Configuration object for all API endpoints
export const apiConfig = {
  baseUrl: API_URL,
  pathPrefix: {
    invalidteDidPrefix: '/did/'
  },
  pathSuffix: {
    createDid: '/did/create',  
    issueCredential: '/credentials/issue',
    invalidateDidSuffix: '/invalidate',
    createPresentation: '/presentations/create',
    requestPresentation: '/presentations/request',
    verifyPresentation: '/presentations/verify',
    // Add other endpoints here as needed
  }
};

// Helper function to get full endpoint URLs
export const getApiUrl = (endpoint) => {
  return `${apiConfig.baseUrl}${endpoint}`;
};

// Export specific endpoint URLs
export const CREATE_DID_URL = getApiUrl(apiConfig.pathSuffix.createDid);
export const ISSUE_CREDENTIAL_URL = getApiUrl(apiConfig.pathSuffix.issueCredential);
export const INVALIDATE_DID_URL = (encodedUrl) => getApiUrl(apiConfig.pathPrefix.invalidteDidPrefix + encodedUrl + apiConfig.pathSuffix.invalidateDidSuffix)
// Export new presentation endpoint URLs
export const CREATE_PRESENTATION_URL = getApiUrl(apiConfig.pathSuffix.createPresentation);
export const REQUEST_PRESENTATION_URL = getApiUrl(apiConfig.pathSuffix.requestPresentation);
export const VERIFY_PRESENTATION_URL = getApiUrl(apiConfig.pathSuffix.verifyPresentation);
