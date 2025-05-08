// Get the API URL from environment variables
// CRA automatically injects environment variables that start with REACT_APP_
const API_URL = process.env.REACT_APP_API_URL;

// Configuration object for all API endpoints
export const apiConfig = {
  baseUrl: API_URL,
  endpoints: {
    createDid: '/did/create',  
    issueCredential: '/credentials/issue',
  }
};

// Helper function to get full endpoint URLs
export const getApiUrl = (endpoint) => {
  return `${apiConfig.baseUrl}${endpoint}`;
};

// Export specific endpoint URLs
export const CREATE_DID_URL = getApiUrl(apiConfig.endpoints.createDid);
export const ISSUE_CREDENTIAL_URL = getApiUrl(apiConfig.endpoints.issueCredential);