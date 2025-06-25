import React, { useState } from "react";
import {
  ISSUE_CREDENTIAL_URL,
  CREATE_PRESENTATION_URL,
  REQUEST_PRESENTATION_URL,
  VERIFY_PRESENTATION_URL,
  CREATE_DID_URL, INVALIDATE_DID_URL
} from './config/api';
import "./App.css";
// import { deserializeDidResponse, getDidString } from './dto/response/CreateDidResp';
import AppUI from './components/AppUI';
import HttpClient from "./http/httpRequestHandler.ts";

function App() {
  const [didMethod, setDidMethod] = useState("key");
  const [ethrNetwork, setEthrNetwork] = useState("none"); // New state for Ethereum network
  const [did, setDid] = useState("");
  const [vc, setVc] = useState("");
  const [presentation, setPresentation] = useState(null);
  const [verificationResult, setVerificationResult] = useState(null);
  const [presentationRequest, setPresentationRequest] = useState(null);
  const [invalidateDid, setInvalidateDid] = useState("");
  const [invalidationResult, setInvalidationResult] = useState(null);

  // Ethereum network options
  const ethrNetworks = [
    {value: "none", label: "Default (Mainnet)", chainId: 1, description: "Creates did:ethr:0x... format"},
    {
      value: "mainnet",
      label: "Ethereum Mainnet (Explicit)",
      chainId: 1,
      description: "Creates did:ethr:mainnet:0x... format"
    },
    {value: "polygon", label: "Polygon", chainId: 137},
    {value: "sepolia", label: "Sepolia Testnet", chainId: 11155111},
    {value: "bsc", label: "Binance Smart Chain", chainId: 56},
    {value: "arbitrum", label: "Arbitrum One", chainId: 42161},
    {value: "optimism", label: "Optimism", chainId: 10},
    {value: "base", label: "Base", chainId: 8453},
    {value: "avalanche", label: "Avalanche C-Chain", chainId: 43114}
  ];

  const keyTypes = {
    ED25519: 'Ed22519',
  };
  const didMethods = {
    ETHR_METHOD: 'ethr',
    KEY_METHOD: 'key',
  };

    /**
   * createDid simulates the creation of a Decentralized Identifier.
   */
  const createDid = async () => {
    try {

      const requestCreateDid = new HttpClient(CREATE_DID_URL)
      const requestBody = {
        method: didMethod,
        keyType: keyTypes.ED25519,
        network: String,
        chainId: Number
      }

      if (didMethod === didMethods.ETHR_METHOD) {
        const selectedNetwork = ethrNetworks.find(network => network.value === ethrNetwork);
        requestBody.network = ethrNetwork;
        requestBody.chainId = selectedNetwork.chainId;
      }

      const response = await requestCreateDid.post(requestBody);

      setDid(response);
      console.log("Created DID:", response)

    } catch (error) {
      console.error("Error creating DID:", error);
    }
  };

  const invalidateDidHandler = async () => {
    try {
      if (!invalidateDid) {
        alert('Please enter a DID to invalidate');
        return;
      }

      const encodedDid = encodeURIComponent(invalidateDid);
      const requestInvalidateDid = new HttpClient(INVALIDATE_DID_URL(encodedDid));
      const response = await requestInvalidateDid.post();

      setInvalidationResult(response);
      console.log("Invalidation Result:", response);

      if (response.success) {
        alert('DID invalidated successfully!');
      } else {
        alert(`Failed to invalidate DID: ${response.message}`);
      }

    } catch (error) {
      console.error("Error invalidating DID:", error);
      
      // Handle specific error types
      if (error.response && error.response.data) {
        const errorData = error.response.data;
        
        switch (errorData.error_type) {
          case 'unsupported_network':
            alert(`❌ Unsupported Network: The network specified in the DID is not supported.`);
            break;
          case 'invalid_did':
            alert(`❌ Invalid DID: ${errorData.message}`);
            break;
          case 'malformed_address':
            alert(`❌ Malformed Address: The Ethereum address in the DID is invalid.`);
            break;
          case 'already_inactive':
            alert(`⚠️ Already Inactive: This DID has already been deactivated.`);
            break;
          case 'did_not_found':
            alert(`❌ DID Not Found: The specified DID could not be found.`);
            break;
          default:
            alert(`❌ Error: ${errorData.message || 'Failed to invalidate DID'}`);
        }
      } else {
        alert(`❌ Network Error: ${error.message || 'Failed to connect to server'}`);
      }
    }
  };

  /**
   * Issues a Verifiable Credential by calling the backend API
   */
  const issueCredential = async () => {
    try {
      if (!did) {
        alert('Please create a DID first');
        return;
      }
      
      const requestIssueCredential = new HttpClient(ISSUE_CREDENTIAL_URL);

      // TODO: temporary data
      const subjectData = {
        name: 'John Doe',
        id: 'did:example:123',
        degree: {
          type: 'BachelorDegree',
          name: 'Bachelor of Science in Computer Science'
        }
      };

      // TODO: Eventually for issuing a vc, we will have support for
      //  a list of different credential types, all of which will need
      //  to be validated by a third party tool
      const response = await requestIssueCredential.post({
        subject: subjectData,
        credential_type: ['UniversityDegreeCredential'],
        issuer_did: 'did:example:issuer',
        expiration_date: null
      });

      setVc(response);
      console.log("Issued Credential:", response);
      alert('Credential issued successfully!');

    } catch (error) {
      console.error("Error issuing credential:", error);
      
      // Handle different types of errors based on the response
      if (error.response && error.response.data) {
        const errorData = error.response.data;
        
        switch (errorData.error_type) {
          case 'invalid_did_status':
            alert(`❌ DID Deactivated: ${errorData.message}`);
            break;
          case 'missing_did':
            alert(`❌ No DID Found: ${errorData.message}`);
            break;
          case 'missing_did_document':
            alert(`❌ DID Document Error: ${errorData.message}`);
            break;
          case 'internal_error':
            alert(`❌ Server Error: ${errorData.message}`);
            break;
          default:
            alert(`❌ Error: ${errorData.message || 'Failed to issue credential'}`);
        }
      } else {
        // Fallback for network errors or unexpected error formats
        alert(`❌ Network Error: ${error.message || 'Failed to connect to server'}`);
      }
    }
  };

  /**
   * Creates a presentation request (verifier side)
   */
      //TODO creation of the presentation request is not created
      // by our wallet application, but rather by our pop up application
  const createPresentationRequest = async () => {
    try {

      const createPresentationRequest = new HttpClient(REQUEST_PRESENTATION_URL)
      const response = await createPresentationRequest.post(
          {
            // TODO: Eventually  we will have support for
            //  a list of different credential types, all of which will need
            //  to be validated by a third party tool
            credential_types: ['UniversityDegreeCredential'],
            fields: [
              ['name', false],  // name is required
              ['degree.name', false]  // degree name is required
            ],
            purpose: 'Verification of university degree'
          }
      );

      setPresentationRequest(response);
      console.log("Created Presentation Request:", response);

    } catch (error) {
      console.error("Error creating presentation request:", error);
    }
  };

  /**
   * Creates a presentation from a credential (holder side)
   */
  const createPresentation = async () => {
    try {
      if (!vc) {
        alert('Please issue a credential first');
        return;
      }
      if (!presentationRequest) {
        alert('Please create a presentation request first');
        return;
      }

      // Extract just the DID string from the DID document
      // const holderDidString = getDidString(did);
      const requestCreatePresentation = new HttpClient(CREATE_PRESENTATION_URL)
      const response = await requestCreatePresentation.post( {
            holder_did: did.id,
            credentials: [vc],
            challenge: presentationRequest.challenge,
            domain: presentationRequest.domain
          }
      );

      // Create a presentation response that includes both the VP and submission metadata
      // This would normally be done by your wallet application
      const presentationSubmission = {
        id: `submission-${Date.now()}`,
        definition_id: presentationRequest.presentation_definition.id,
        descriptor_map: [
          {
            id: presentationRequest.presentation_definition.input_descriptors[0].id,
            format: "ldp_vp",
            path: "$.verifiableCredential[0]"
          }
        ]
      };

      const presentationResponse = {
        verifiable_presentation: response,
        presentation_submission: presentationSubmission
      };

      setPresentation(presentationResponse);
      console.log("Created Presentation:", presentationResponse);

    } catch (error) {
      console.error("Error creating presentation:", error);
    }
  };

  /**
   * Verifies a presentation (verifier side)
   */
  const verifyPresentation = async () => {
    try {
      if (!presentation || !presentationRequest) {
        alert('Please create both a presentation request and a presentation first');
        return;
      }

      const requestVerifyPresentation = new HttpClient(VERIFY_PRESENTATION_URL)
      const response = await requestVerifyPresentation.post(
          {
            original_request: presentationRequest,
            presentation_response: presentation
          }
      )
      const result = await response;
      setVerificationResult(response);

      if (result.is_valid) {
        alert('Presentation verified successfully!');
      } else {
        alert('Presentation verification failed');
      }

    } catch (error) {
      console.error("Error verifying presentation:", error);
    }
  };

  // Return the UI component with all necessary props
  return (
      <AppUI
          did={did}
          vc={vc}
          presentationRequest={presentationRequest}
          presentation={presentation}
          verificationResult={verificationResult}
          createDid={createDid}
          didMethod={didMethod}
          ethrNetwork={ethrNetwork}
          setEthrNetwork={setEthrNetwork}
          ethrNetworks={ethrNetworks}
          setDidMethod={setDidMethod}
          issueCredential={issueCredential}
          createPresentationRequest={createPresentationRequest}
          createPresentation={createPresentation}
          verifyPresentation={verifyPresentation}
          invalidateDid={invalidateDid}
          invalidationResult={invalidationResult}
          setInvalidateDid={setInvalidateDid}
          invalidateDidHandler={invalidateDidHandler}
      />
  );
}

export default App;