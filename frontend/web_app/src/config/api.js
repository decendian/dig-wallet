// Get the API URL from environment variables
// CRA automatically injects environment variables that start with REACT_APP_
const API_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';

// Configuration object for all API endpoints
export const apiConfig = {
  baseUrl: API_URL,
  endpoints: {
    issueCredential: '/credentials/issue',
    // New presentation endpoints
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
export const ISSUE_CREDENTIAL_URL = getApiUrl(apiConfig.endpoints.issueCredential);
// Export new presentation endpoint URLs
export const CREATE_PRESENTATION_URL = getApiUrl(apiConfig.endpoints.createPresentation);
export const REQUEST_PRESENTATION_URL = getApiUrl(apiConfig.endpoints.requestPresentation);
export const VERIFY_PRESENTATION_URL = getApiUrl(apiConfig.endpoints.verifyPresentation);
