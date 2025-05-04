import React, { useState } from "react";
import { ISSUE_CREDENTIAL_URL, CREATE_DID_URL} from './config/api';
import "./App.css";

function App() {
  // Local state for our dummy outputs
  const [did, setDid] = useState("");
  const [vc, setVc] = useState("");
  const [age, setAge] = useState("");
  const [error, setError] = useState(null);


  /**
   * createDid simulates the creation of a Decentralized Identifier.
   * TODO: Replace stub with a backend API call to create a DID.
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
      // Add a loading state if you don't have one
      // setIsLoading(true);
      
      // Sample subject data
      const subjectData = {
        name: 'John Doe',
        id: 'did:example:123',
        degree: {
          type: 'BachelorDegree',
          name: 'Bachelor of Science in Computer Science'
        }
      };
      
      // Make API call to your Rust backend
      const response = await fetch(ISSUE_CREDENTIAL_URL, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
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
      // setError(error.message);
    } finally {
      // setIsLoading(false);
    }
  };

  /**
   * verifyAge simulates a zero-knowledge proof based age verification.
   * TODO: Replace this stub with an integration to your ZKP engine.
   */
  const verifyAge = () => {
    // For demo purposes, we simply check if the entered age is 21 or older
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
      
      {/* Section 3: Age Verification */}
      <div className="section">
        <h2>3. Age Verification (ZKP Stub)</h2>
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
          Note: The backend endpoints and cryptographic operations (DID generation,
          VC issuance, ZKP validation) are currently stubbed. Please integrate your backend
          logic and security measures as needed.
        </p>
      </div>
    </div>
  );
}

export default App;
