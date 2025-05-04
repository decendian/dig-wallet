import React, { useState } from "react";
import { 
  ISSUE_CREDENTIAL_URL, 
  CREATE_PRESENTATION_URL,
  REQUEST_PRESENTATION_URL, 
  VERIFY_PRESENTATION_URL 
} from './config/api';
import { ISSUE_CREDENTIAL_URL, CREATE_DID_URL} from './config/api';
import "./App.css";

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
      // Optional: Add loading state
      // setIsLoading(true);

      // Call your backend API
      const response = await fetch(CREATE_DID_URL, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          // Optional: Include any parameters your backend needs
          // For example, you might want to specify key_type
          keyType: 'Ed25519',  // or 'Secp256k1' or 'P256'
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
      }

      // Parse the response
      const did = await response.json();

      // Based on your backend handler.rs, the response should contain a DIDDocument
      // Extract the 'id' field which contains the DID
      // Update state
      setDid(did);
      console.log("Created DID:", did);

    } catch (error) {
      console.error("Error creating DID:", error);
      // Optional: set error state
      // setError(error.message);
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
      
      const response = await fetch(CREATE_PRESENTATION_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          holder_did: did || 'did:example:holder',
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

  return (
    <div className="App">
      <h1>Decentralized Identity Guardian MVP</h1>
      
      {/* Section 1: DID Creation */}
      <div className="section">
        <h2>1. Create Decentralized Identifier (DID)</h2>
        <button onClick={createDid}>Create DID</button>
        {did &&
            <div>
              <p>Your DID:</p>
              <pre> {JSON.stringify(did, null, 2)} </pre>
            </div>}
      </div>
      
      {/* Section 2: Credential Issuance */}
      <div className="section">
        <h2>2. Issue Verifiable Credential (VC)</h2>
        <button onClick={issueCredential}>Issue Credential</button>
        {vc && (
          <div>
            <p>Your VC:</p>
            <pre>{JSON.stringify(vc, null, 2)}</pre>
          </div>
        )}
      </div>
      
      {/* New Section: Presentation Request */}
      <div className="section">
        <h2>3. Create Presentation Request (Verifier)</h2>
        <button onClick={createPresentationRequest}>Create Request</button>
        {presentationRequest && (
          <div>
            <p>Presentation Request:</p>
            <pre>{JSON.stringify(presentationRequest, null, 2)}</pre>
          </div>
        )}
      </div>
      
      {/* New Section: Create Presentation */}
      <div className="section">
        <h2>4. Create Presentation (Holder)</h2>
        <button onClick={createPresentation}>Create Presentation</button>
        {presentation && (
          <div>
            <p>Your Presentation:</p>
            <pre>{JSON.stringify(presentation, null, 2)}</pre>
          </div>
        )}
      </div>
      
      {/* New Section: Verify Presentation */}
      <div className="section">
        <h2>5. Verify Presentation (Verifier)</h2>
        <button onClick={verifyPresentation}>Verify Presentation</button>
        {verificationResult && (
          <div>
            <p>Verification Result:</p>
            <pre>{JSON.stringify(verificationResult, null, 2)}</pre>
          </div>
        )}
      </div>
      
      {/* Section: Age Verification */}
      <div className="section">
        <h2>6. Age Verification (ZKP Stub)</h2>
        <input
          type="number"
          placeholder="Enter your age"
          value={age}
          onChange={(e) => setAge(e.target.value)}
        />
        <button onClick={verifyAge}>Verify Age</button>
      </div>
      
      {/* Info section */}
      <div className="section">
        <p>
          Note: This application demonstrates the full DID presentation exchange flow:
          1. Create a DID
          2. Issue a Verifiable Credential
          3. Request a Presentation (verifier)
          4. Create a Presentation (holder)
          5. Verify the Presentation (verifier)
        </p>
      </div>
    </div>
  );
}

export default App;