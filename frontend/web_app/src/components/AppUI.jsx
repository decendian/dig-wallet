// frontend/web_app/src/components/AppUI.jsx

import React from 'react';

const AppUI = ({
  did,
  vc,
  age,
  presentationRequest,
  presentation,
  verificationResult,
  createDid,
  issueCredential,
  createPresentationRequest,
  createPresentation,
  verifyPresentation,
  verifyAge,
  setAge
}) => {
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
};

export default AppUI;