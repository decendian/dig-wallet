import React, { useState } from 'react';
import './AppUI.css';

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
  const [activeTab, setActiveTab] = useState('create');
  
  const renderTabContent = () => {
    switch(activeTab) {
      case 'create':
        return (
          <div className="tab-content">
            <h2>Create Decentralized Identifier (DID)</h2>
            <div className="card">
              <div className="form-group">
                <label>DID Method</label>
                <select
                  value={didMethod}
                  onChange={(e) => setDidMethod(e.target.value)}
                  className="select-input"
                >
                  <option value="key">Key DID</option>
                  <option value="ethr">Ethereum DID</option>
                </select>
              </div>

              {didMethod === "ethr" && (
                <div className="form-group">
                  <label>Network</label>
                  <select
                    value={ethrNetwork}
                    onChange={(e) => setEthrNetwork(e.target.value)}
                    className="select-input"
                  >
                    {ethrNetworks.map(network => (
                      <option key={network.value} value={network.value}>
                        {network.label}
                      </option>
                    ))}
                  </select>

                  {
                    () => {
                      return (
                          <>
                            <div className="info-box">

                              <p className="info-title">
                                Selected: {ethrNetworks.find(n => n.value === ethrNetwork)?.label}
                                {ethrNetworks.find(n => n.value === ethrNetwork)?.chainId &&
                                    ` (Chain ID: ${ethrNetworks.find(n => n.value === ethrNetwork)?.chainId})`
                                }
                              </p>

                              <p className="info-detail">
                                {ethrNetwork === 'none'
                                    ? "Will create: did:ethr:0x... (classic format, defaults to mainnet)"
                                    : `Will create: did:ethr:${ethrNetwork}:0x... (network-specific format)`
                                }
                              </p>
                            </div>
                          </>
                      );
                    }
                  }
                </div>
              )}

              <button 
                className="primary-button" 
                onClick={createDid}
              >
                Create {didMethod === "ethr" ?
                  (ethrNetwork === "none" ? "Default Ethereum " : `${ethrNetworks.find(n => n.value === ethrNetwork)?.label} `)
                  : "" }DID
              </button>
            </div>

            {did && (
              <div className="result-container">
                <h3>Your DID</h3>
                <div className="json-result">
                  <pre>{JSON.stringify(did, null, 2)}</pre>
                </div>
              </div>
            )}
          </div>
        );
      
      case 'invalidate':
        return (
          <div className="tab-content">
            <h2>Invalidate DID</h2>
            <div className="card">
              <div className="form-group">
                <label>DID to invalidate</label>
                <input
                  type="text"
                  placeholder="Enter DID (e.g., did:key:z6Mk..., did:ethr:0x...)"
                  value={invalidateDid}
                  onChange={(e) => setInvalidateDid(e.target.value)}
                  className="text-input"
                />
              </div>
              <button 
                className="danger-button" 
                onClick={invalidateDidHandler}
              >
                Invalidate DID
              </button>
            </div>

            {invalidationResult && (
              <div className="result-container">
                <h3>Invalidation Result</h3>
                <div className={`json-result ${invalidationResult.success ? 'success' : 'error'}`}>
                  <pre>{JSON.stringify(invalidationResult, null, 2)}</pre>
                </div>
              </div>
            )}
          </div>
        );
      
      case 'issue':
        return (
          <div className="tab-content">
            <h2>Issue Verifiable Credential (VC)</h2>
            <div className="card">
              <p className="instruction">Create a verifiable credential using your DID as the issuer.</p>
              <button 
                className="primary-button"
                onClick={issueCredential}
                disabled={!did}
              >
                Issue Credential
              </button>
              {!did && <p className="warning">Please create a DID first</p>}
            </div>

            {vc && (
              <div className="result-container">
                <h3>Your Verifiable Credential</h3>
                <div className="json-result">
                  <pre>{JSON.stringify(vc, null, 2)}</pre>
                </div>
              </div>
            )}
          </div>
        );
      
      case 'request':
        return (
          <div className="tab-content">
            <h2>Create Presentation Request (Verifier)</h2>
            <div className="card">
              <p className="instruction">Generate a request for presentation as a verifier.</p>
              <button 
                className="primary-button" 
                onClick={createPresentationRequest}
              >
                Create Request
              </button>
            </div>

            {presentationRequest && (
              <div className="result-container">
                <h3>Presentation Request</h3>
                <div className="json-result">
                  <pre>{JSON.stringify(presentationRequest, null, 2)}</pre>
                </div>
              </div>
            )}
          </div>
        );
      
      case 'present':
        return (
          <div className="tab-content">
            <h2>Create Presentation (Holder)</h2>
            <div className="card">
              <p className="instruction">Generate a presentation in response to a request using your credential.</p>
              <button 
                className="primary-button" 
                onClick={createPresentation}
                disabled={!vc || !presentationRequest}
              >
                Create Presentation
              </button>
              {(!vc || !presentationRequest) && 
                <p className="warning">Please issue a credential and create a presentation request first</p>
              }
            </div>

            {presentation && (
              <div className="result-container">
                <h3>Your Presentation</h3>
                <div className="json-result">
                  <pre>{JSON.stringify(presentation, null, 2)}</pre>
                </div>
              </div>
            )}
          </div>
        );
      
      case 'verify':
        return (
          <div className="tab-content">
            <h2>Verify Presentation (Verifier)</h2>
            <div className="card">
              <p className="instruction">Verify the presentation as a verifier.</p>
              <button 
                className="primary-button" 
                onClick={verifyPresentation}
                disabled={!presentation}
              >
                Verify Presentation
              </button>
              {!presentation && <p className="warning">Please create a presentation first</p>}
            </div>

            {verificationResult && (
              <div className="result-container">
                <h3>Verification Result</h3>
                <div className={`json-result ${verificationResult.verified ? 'success' : 'error'}`}>
                  <pre>{JSON.stringify(verificationResult, null, 2)}</pre>
                </div>
              </div>
            )}
          </div>
        );
      
      default:
        return null;
    }
  };

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>Decentralized Identity Guardian</h1>
        <p className="subtitle">Self-Sovereign Identity Management System</p>
      </header>

      <main className="app-content">
        <nav className="app-nav">
          <ul>
            <li 
              className={activeTab === 'create' ? 'active' : ''} 
              onClick={() => setActiveTab('create')}
            >
              <span className="nav-icon">🔑</span>
              Create DID
            </li>
            <li 
              className={activeTab === 'invalidate' ? 'active' : ''} 
              onClick={() => setActiveTab('invalidate')}
            >
              <span className="nav-icon">🚫</span>
              Invalidate DID
            </li>
            <li 
              className={activeTab === 'issue' ? 'active' : ''} 
              onClick={() => setActiveTab('issue')}
            >
              <span className="nav-icon">📝</span>
              Issue Credential
            </li>
            <li 
              className={activeTab === 'request' ? 'active' : ''} 
              onClick={() => setActiveTab('request')}
            >
              <span className="nav-icon">📤</span>
              Request Presentation
            </li>
            <li 
              className={activeTab === 'present' ? 'active' : ''} 
              onClick={() => setActiveTab('present')}
            >
              <span className="nav-icon">📋</span>
              Create Presentation
            </li>
            <li 
              className={activeTab === 'verify' ? 'active' : ''} 
              onClick={() => setActiveTab('verify')}
            >
              <span className="nav-icon">✅</span>
              Verify Presentation
            </li>
          </ul>
        </nav>

        <section className="content-area">
          {renderTabContent()}
        </section>
      </main>

      <footer className="app-footer">
        <div className="flow-diagram">
          <div className="flow-step">
            <span className="step-number">1</span>
            <span className="step-text">Create DID</span>
          </div>
          <div className="flow-arrow">→</div>
          <div className="flow-step">
            <span className="step-number">2</span>
            <span className="step-text">Issue VC</span>
          </div>
          <div className="flow-arrow">→</div>
          <div className="flow-step">
            <span className="step-number">3</span>
            <span className="step-text">Request</span>
          </div>
          <div className="flow-arrow">→</div>
          <div className="flow-step">
            <span className="step-number">4</span>
            <span className="step-text">Present</span>
          </div>
          <div className="flow-arrow">→</div>
          <div className="flow-step">
            <span className="step-number">5</span>
            <span className="step-text">Verify</span>
          </div>
        </div>
      </footer>
    </div>
  );
};

export default AppUI;