import React, { useState } from "react";
import {
  ISSUE_CREDENTIAL_URL,
  CREATE_PRESENTATION_URL,
  REQUEST_PRESENTATION_URL,
  VERIFY_PRESENTATION_URL,
  CREATE_DID_URL
} from './config/api';
import "./App.css";
import { createDidRequest, validateDidRequest } from './dto/request/CreateDidReq';
import { deserializeDidResponse, getDidString } from './dto/response/CreateDidResp';
import AppUI from './components/AppUI';

function App() {
  const [did, setDid] = useState("");
  const [vc, setVc] = useState("");
  const [age, setAge] = useState("");
  const [error, setError] = useState(null);


  // New state for presentation exchange
  const [presentationRequest, setPresentationRequest] = useState(null);
  const [presentation, setPresentation] = useState(null);
  const [verificationResult, setVerificationResult] = useState(null);

  /**
   * createDid simulates the creation of a Decentralized Identifier.
   */
  const createDid = async () => {
    try {
      // Create the request DTO
      const requestDto = createDidRequest(
          'ethr',  // Default method
          'Ed25519', // Default key type
          { network: 'testnet' } // Optional parameters
      );

      // Validate the request
      if (!validateDidRequest(requestDto)) {
        throw new Error("Invalid DID request parameters");
      }

      // Call your backend API with the properly formatted request
      const response = await fetch(CREATE_DID_URL, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(requestDto),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to create DID');
      }

      // Parse the response using the response DTO utilities
      const responseText = await response.text();
      const didResponse = deserializeDidResponse(responseText);

      // Update state with the complete DID document
      setDid(didResponse);
      console.log("Created DID:", didResponse);
      console.log("DID Identifier:", getDidString(didResponse));

    } catch (error) {
      console.error("Error creating DID:", error);
      setError(error.message);
    }
  };

  /**
   * Issues a Verifiable Credential by calling the backend API
   */
  const issueCredential = async () => {
    try {
      const subjectData = {
        name: 'John Doe',
        id: 'did:example:123',
        degree: {
          type: 'BachelorDegree',
          name: 'Bachelor of Science in Computer Science'
        }
      };

      const response = await fetch(ISSUE_CREDENTIAL_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          subject: subjectData,
          credential_type: ['UniversityDegreeCredential'],
          issuer_did: 'did:example:issuer',
          expiration_date: null
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to issue credential');
      }

      const credential = await response.json();
      setVc(credential);
      console.log("Issued Credential:", credential);

    } catch (error) {
      console.error("Error issuing credential:", error);
    }
  };

  /**
   * Creates a presentation request (verifier side)
   */
  const createPresentationRequest = async () => {
    try {
      const response = await fetch(REQUEST_PRESENTATION_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          credential_types: ['UniversityDegreeCredential'],
          fields: [
            ['name', false],  // name is required
            ['degree.name', false]  // degree name is required
          ],
          purpose: 'Verification of university degree'
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to create presentation request');
      }

      const request = await response.json();
      setPresentationRequest(request);
      console.log("Created Presentation Request:", request);

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
      const holderDidString = did ? getDidString(did) : 'did:example:holder';

      const response = await fetch(CREATE_PRESENTATION_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          holder_did: holderDidString,  // Use the string, not the object
          credentials: [vc],
          challenge: presentationRequest.challenge,
          domain: presentationRequest.domain
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to create presentation');
      }

      const vp = await response.json();

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
        verifiable_presentation: vp,
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

      const response = await fetch(VERIFY_PRESENTATION_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          original_request: presentationRequest,
          presentation_response: presentation
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to verify presentation');
      }

      const result = await response.json();
      setVerificationResult(result);
      console.log("Verification Result:", result);

      if (result.is_valid) {
        alert('Presentation verified successfully!');
      } else {
        alert('Presentation verification failed');
      }

    } catch (error) {
      console.error("Error verifying presentation:", error);
    }
  };

  /**
   * ZKP age verification stub
   */
  const verifyAge = () => {
    if (parseInt(age, 10) >= 21) {
      alert("Age verification successful!");
    } else {
      alert("Age verification failed. Must be 21+");
    }
  };

  // Return the UI component with all necessary props
  return (
      <AppUI
          did={did}
          vc={vc}
          age={age}
          presentationRequest={presentationRequest}
          presentation={presentation}
          verificationResult={verificationResult}
          createDid={createDid}
          issueCredential={issueCredential}
          createPresentationRequest={createPresentationRequest}
          createPresentation={createPresentation}
          verifyPresentation={verifyPresentation}
          verifyAge={verifyAge}
          setAge={setAge}
      />
  );
}

export default App;