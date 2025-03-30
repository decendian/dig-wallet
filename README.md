# dig-for-security
Decentralized Identity Guardian. Part of Web3.0, foundation is built on Hedera Blockchain. A platform for secure decentralized digital identity. It provides secure storage, key management, verifiable credentials, and access control policies. 

### Package structure for this platform

``` 
decentralized-identity-guardian/
│── Cargo.toml                # Rust workspace configuration
│── Cargo.lock
│── README.md
│── rust-toolchain.toml        # (Optional, pinning Rust version)
│
├── crates/                    # Core Rust components
│   ├── did-library/           # Your DID library (pluggable component)
│   ├── identity-wallet/       # Secure storage & key management
│   ├── access-control/        # Guardian policies, verification, authentication
│   ├── verifiable-credentials/ # VC issuance and verification
│
├── services/                  # API and CLI tools
│   ├── api-server/            # REST/gRPC API for frontend & mobile clients
│   ├── cli/                   # Command-line client for identity management
│
├── frontend/                   # Web & mobile interfaces
│   ├── web-app/                # Web frontend (React, Next.js, or Yew)
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── src/
│   │   ├── public/
│   │   ├── README.md
│   │
│   ├── mobile-app/             # Mobile frontend (Flutter, React Native, etc.)
│   │   ├── pubspec.yaml
│   │   ├── lib/
│   │   ├── android/
│   │   ├── ios/
│   │   ├── README.md
│
└── deployment/                 # Deployment scripts (Docker, Kubernetes, CI/CD)
    ├── docker/
    ├── terraform/
```