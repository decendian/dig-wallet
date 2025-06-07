import React, { useState } from "react";
import { 
  ISSUE_CREDENTIAL_URL, 
  CREATE_PRESENTATION_URL,
  REQUEST_PRESENTATION_URL, 
  VERIFY_PRESENTATION_URL,
  CREATE_DID_URL 
} from './config/api';
import "./App.css";

function App() {
  const [didMethod, setDidMethod] = useState("key");
  const [ethrNetwork, setEthrNetwork] = useState("none"); // New state for Ethereum network
  const [did, setDid] = useState("");
  const [vc, setVc] = useState("");
  const [age, setAge] = useState("");
  const [error, setError] = useState(null);

  
  // New state for presentation exchange
  const [presentationRequest, setPresentationRequest] = useState(null);
  const [presentation, setPresentation] = useState(null);
  const [verificationResult, setVerificationResult] = useState(null);

  const [invalidateDid, setInvalidateDid] = useState("");
  const [invalidationResult, setInvalidationResult] = useState(null);

  // Ethereum network options
  const ethrNetworks = [
    { value: "none", label: "Default (Mainnet)", chainId: 1, description: "Creates did:ethr:0x... format" },
    { value: "mainnet", label: "Ethereum Mainnet (Explicit)", chainId: 1, description: "Creates did:ethr:mainnet:0x... format" },
    { value: "polygon", label: "Polygon", chainId: 137 },
    { value: "sepolia", label: "Sepolia Testnet", chainId: 11155111 },
    { value: "bsc", label: "Binance Smart Chain", chainId: 56 },
    { value: "arbitrum", label: "Arbitrum One", chainId: 42161 },
    { value: "optimism", label: "Optimism", chainId: 10 },
    { value: "base", label: "Base", chainId: 8453 },
    { value: "avalanche", label: "Avalanche C-Chain", chainId: 43114 }
  ];

  /**
   * createDid simulates the creation of a Decentralized Identifier.
   */
  const createDid = async () => {
  try {
      // Optional: Add loading state
      // setIsLoading(true);

      // Prepare request body based on selected method
      const requestBody = {
        method: didMethod,
        keyType: 'Ed25519',  // or 'Secp256k1' or 'P256'
        network: String,
        chainId: Number
      };

      // Add network information for Ethereum DIDs
      if (didMethod === 'ethr') {
        const selectedNetwork = ethrNetworks.find(network => network.value === ethrNetwork);
        
        if (ethrNetwork !== 'none') {
          // Include network for explicit network selection
          requestBody.network = ethrNetwork;
          requestBody.chainId = selectedNetwork.chainId;
        } else {
          // For "none", don't include network (defaults to mainnet)
          // This creates the classic did:ethr:0x... format
          requestBody.useDefaultNetwork = true;
        }
        
        // TODO: Backend needs to support network-specific endpoints
        // Example: POST /api/did/ethr/[network]/create
        // For "none" option: POST /api/did/ethr/create (no network in path)
        // Or include network in the request body as we're doing here
      }

      // Call your backend API
      const response = await fetch(CREATE_DID_URL, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(requestBody),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to create DID');
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
      setError(error.message);
    }
  };

  const invalidateDidHandler = async () => {
  try {
    if (!invalidateDid) {
      alert('Please enter a DID to invalidate');
      return;
    }

    const encodedDid = encodeURIComponent(invalidateDid);
    
    // TODO: Backend might need network-specific invalidation endpoints
    // For Ethereum DIDs, you might want to extract the network from the DID
    // or add network selection for invalidation as well
    // Example: POST /api/did/ethr/[network]/[did]/invalidate
    
    const response = await fetch(`http://localhost:8080/api/did/${encodedDid}/invalidate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      const errorData = await response.json();
      throw new Error(errorData.message || 'Failed to invalidate DID');
    }

    const result = await response.json();
    setInvalidationResult(result);
    console.log("Invalidation Result:", result);

    if (result.success) {
      alert('DID invalidated successfully!');
    } else {
      alert(`Failed to invalidate DID: ${result.message}`);
    }

  } catch (error) {
    console.error("Error invalidating DID:", error);
    alert(`Error: ${error.message}`);
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
      const holderDidString = did ? did.id : 'did:example:holder';
      
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

  return (
    <div className="App">
      <h1>Decentralized Identity Guardian MVP</h1>
      
      {/* Section 1: DID Creation */}
      <div className="section">
        <h2>1. Create Decentralized Identifier (DID)</h2>
        <div className="method-selector">
          <label>
            DID Method:
            <select 
              value={didMethod} 
              onChange={(e) => setDidMethod(e.target.value)}
            >
              <option value="key">Key DID</option>
              <option value="ethr">Ethr DID</option>
            </select>
          </label>
          
          {/* Network selector - only show when Ethr DID is selected */}
          {didMethod === "ethr" && (
            <label style={{ marginLeft: '20px' }}>
              Network:
              <select 
                value={ethrNetwork} 
                onChange={(e) => setEthrNetwork(e.target.value)}
              >
                {ethrNetworks.map(network => (
                  <option key={network.value} value={network.value}>
                    {network.label}
                  </option>
                ))}
              </select>
            </label>
          )}
        </div>
        
        {/* Display selected configuration */}
        {didMethod === "ethr" && (
          <div style={{ 
            marginTop: '10px', 
            padding: '8px', 
            backgroundColor: '#f0f8ff', 
            borderRadius: '4px',
            fontSize: '14px'
          }}>
            Selected: {ethrNetworks.find(n => n.value === ethrNetwork)?.label} 
            (Chain ID: {ethrNetworks.find(n => n.value === ethrNetwork)?.chainId})
            {ethrNetwork === 'none' && (
              <div style={{ fontSize: '12px', color: '#666', marginTop: '4px' }}>
                Will create: did:ethr:0x... (classic format, defaults to mainnet)
              </div>
            )}
            {ethrNetwork !== 'none' && (
              <div style={{ fontSize: '12px', color: '#666', marginTop: '4px' }}>
                Will create: did:ethr:{ethrNetwork}:0x... (network-specific format)
              </div>
            )}
          </div>
        )}
        
        <button onClick={createDid} style={{ marginTop: '10px' }}>
          Create {didMethod === "ethr" ? 
            (ethrNetwork === "none" ? "Default Ethereum " : `${ethrNetworks.find(n => n.value === ethrNetwork)?.label} `) 
            : ""}DID
        </button>
        {did &&
          <div>
            <p>Your DID:</p>
            <pre>{JSON.stringify(did, null, 2)}</pre>
          </div>
        }
      </div>

      {/* Section 2: DID Invalidation */}
      <div className="section">
        <h2>2. Invalidate DID</h2>
        <input
          type="text"
          placeholder="Enter DID to invalidate (e.g., did:key:z6Mk..., did:ethr:0x..., or did:ethr:polygon:0x...)"
          value={invalidateDid}
          onChange={(e) => setInvalidateDid(e.target.value)}
          style={{ width: '400px', padding: '8px', marginRight: '10px' }}
        />
        <button onClick={invalidateDidHandler} style={{ backgroundColor: '#ff6b6b' }}>
          Invalidate DID
        </button>
        
        {invalidationResult && (
          <div style={{ marginTop: '15px' }}>
            <p>Invalidation Result:</p>
            <pre style={{ 
              backgroundColor: invalidationResult.success ? '#d4edda' : '#f8d7da',
              padding: '10px',
              borderRadius: '4px'
            }}>
              {JSON.stringify(invalidationResult, null, 2)}
            </pre>
          </div>
        )}
      </div>
      
      {/* Section 3: Credential Issuance */}
      <div className="section">
        <h2>3. Issue Verifiable Credential (VC)</h2>
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
        <h2>4. Create Presentation Request (Verifier)</h2>
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
        <h2>5. Create Presentation (Holder)</h2>
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
        <h2>6. Verify Presentation (Verifier)</h2>
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
        <h2>7. Age Verification (ZKP Stub)</h2>
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
          1. Create a DID (now with network selection for Ethereum DIDs)
          2. Issue a Verifiable Credential
          3. Request a Presentation (verifier)
          4. Create a Presentation (holder)
          5. Verify the Presentation (verifier)
        </p>
        
        {/* TODO: Backend implementation notes */}
        <div style={{ 
          marginTop: '20px', 
          padding: '15px', 
          backgroundColor: '#fff3cd', 
          borderRadius: '4px',
          border: '1px solid #ffeaa7'
        }}>
          <h4>🚧 Backend Implementation Needed:</h4>
          <ul style={{ textAlign: 'left', margin: '10px 0' }}>
            <li><strong>DID Creation:</strong> Update CREATE_DID_URL endpoint to handle network parameter for Ethereum DIDs</li>
            <li><strong>DID Resolution:</strong> Implement network-specific DID resolution for different chains</li>
            <li><strong>DID Invalidation:</strong> Update invalidation endpoint to handle network-specific Ethereum DIDs</li>
            <li><strong>Registry Storage:</strong> Store network information with DID documents</li>
            <li><strong>Alternative Endpoint Structure:</strong> Consider endpoints like `/api/did/ethr/[network]/create` for explicit networks and `/api/did/ethr/create` for default format</li>
          </ul>
        </div>
      </div>
    </div>
  );
}

export default App;