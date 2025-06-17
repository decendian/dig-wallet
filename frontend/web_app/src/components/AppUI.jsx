// frontend/web_app/src/components/AppUI.jsx

import React from 'react';

const AppUI = ({
  did,
  vc,
  presentationRequest,
  presentation,
  verificationResult,
  createDid,
  didMethod,
  ethrNetwork,
  setEthrNetwork,
  ethrNetworks,
  setDidMethod,
  issueCredential,
  createPresentationRequest,
  createPresentation,
  verifyPresentation,
  invalidateDid,
  invalidationResult,
  setInvalidateDid,
  invalidateDidHandler,
}) => {

  return (
      <div className="App">
        <h1>Decentralized Identity Guardian MVP</h1>

        {/* Section 1: DID Creation */}
        <div className="section">
          <h2>Create Decentralized Identifier (DID)</h2>
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
                <label style={{marginLeft: '20px'}}>
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
                    <div style={{fontSize: '12px', color: '#666', marginTop: '4px'}}>
                      Will create: did:ethr:0x... (classic format, defaults to mainnet)
                    </div>
                )}
                {ethrNetwork !== 'none' && (
                    <div style={{fontSize: '12px', color: '#666', marginTop: '4px'}}>
                      Will create: did:ethr:{ethrNetwork}:0x... (network-specific format)
                    </div>
                )}
              </div>
          )}

          <button onClick={createDid} style={{marginTop: '10px'}}>
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
          <h2>Invalidate DID</h2>
          <input
              type="text"
              placeholder="Enter DID to invalidate (e.g., did:key:z6Mk..., did:ethr:0x..., or did:ethr:polygon:0x...)"
              value={invalidateDid}
              onChange={(e) => setInvalidateDid(e.target.value)}
              style={{width: '400px', padding: '8px', marginRight: '10px'}}
          />
          <button onClick={invalidateDidHandler} style={{backgroundColor: '#ff6b6b'}}>
            Invalidate DID
          </button>

          {invalidationResult && (
              <div style={{marginTop: '15px'}}>
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
          <h2>Issue Verifiable Credential (VC)</h2>
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
          <h2>Create Presentation Request (Verifier)</h2>
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
          <h2>Create Presentation (Holder)</h2>
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
          <h2>Verify Presentation (Verifier)</h2>
          <button onClick={verifyPresentation}>Verify Presentation</button>
          {verificationResult && (
              <div>
                <p>Verification Result:</p>
                <pre>{JSON.stringify(verificationResult, null, 2)}</pre>
              </div>
          )}
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

        </div>
      </div>
  );
}
export default AppUI;