import React, { useState } from "react";
import "./App.css";

function App() {
  // Local state for our dummy outputs
  const [did, setDid] = useState("");
  const [vc, setVc] = useState("");
  const [age, setAge] = useState("");

  /**
   * createDid simulates the creation of a Decentralized Identifier.
   * TODO: Replace stub with a backend API call to create a DID.
   */
  const createDid = () => {
    // Dummy DID - in production, you’d call your backend and use a real DID generation mechanism
    const dummyDid = "did:hedera:12345";
    setDid(dummyDid);
    console.log("Created DID:", dummyDid);
  };

  /**
   * issueCredential simulates the issuance of a Verifiable Credential.
   * TODO: Replace stub with a backend API call to issue a credential.
   */
  const issueCredential = () => {
    // Dummy VC - in production, this should be generated and signed by an issuer
    const dummyVc = "VC: { name: 'John Doe', credential: 'University Degree' }";
    setVc(dummyVc);
    console.log("Issued Credential:", dummyVc);
  };

  /**
   * verifyAge simulates a zero-knowledge proof based age verification.
   * TODO: Replace this stub with an integration to your ZKP engine.
   */
  const verifyAge = () => {
    // For demo purposes, we simply check if the entered age is 21 or older
    if (parseInt(age, 10) >= 21) {
      alert("Age verification successful (stub)!");
    } else {
      alert("Age verification failed. Must be 21+ (stub).");
    }
  };

  return (
    <div className="App">
      <h1>Decentralized Identity Guardian MVP</h1>
      
      {/* Section 1: DID Creation */}
      <div className="section">
        <h2>1. Create Decentralized Identifier (DID)</h2>
        <button onClick={createDid}>Create DID</button>
        {did && <p>Your DID: {did}</p>}
      </div>
      
      {/* Section 2: Credential Issuance */}
      <div className="section">
        <h2>2. Issue Verifiable Credential (VC)</h2>
        <button onClick={issueCredential}>Issue Credential</button>
        {vc && <p>Your VC: {vc}</p>}
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
