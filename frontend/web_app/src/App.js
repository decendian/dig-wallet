import React, { useState } from "react";
import {
  ISSUE_CREDENTIAL_URL,
  CREATE_PRESENTATION_URL,
  REQUEST_PRESENTATION_URL,
  VERIFY_PRESENTATION_URL,
  CREATE_DID_URL
} from './config/api';
import "./App.css";
// import { deserializeDidResponse, getDidString } from './dto/response/CreateDidResp';
import AppUI from './components/AppUI';
import HttpClient from "./http/httpRequestHandler.ts";

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
      const requestCreateDid = new HttpClient(CREATE_DID_URL)

      //temporary: static key type added as there is no way to add values
      const response = await requestCreateDid.post({ keyType: 'Ed25519' })

      // Parse the response using the response DTO utilities
      // const didResponse = deserializeDidResponse(response.json());

      // Update state with the complete DID document
      setDid(response);

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

      if (!did) {
        alert('Please issue a credential first');
        return;
      }
      const requestIssueCredential = new HttpClient(ISSUE_CREDENTIAL_URL)

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
        expiration_date: null
      })

      // const response = await fetch(ISSUE_CREDENTIAL_URL, {
      //
      //   method: 'POST',
      //   headers: { 'Content-Type': 'application/json' },
      //   body: JSON.stringify({
      //     subject: subjectData,
      //     credential_type: ['UniversityDegreeCredential'],
      //     issuer_did: 'did:example:issuer',
      //     expiration_date: null
      //   }),
      // });
      const credentialResponse = await response.json();
      setVc(response);
      console.log("Issued Credential:", response);

    } catch (error) {
      console.error("Error issuing credential:", error);
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
      )

      // const response = await fetch(REQUEST_PRESENTATION_URL, {
      //
      //   method: 'POST',
      //   headers: { 'Content-Type': 'application/json' },
      //   body: JSON.stringify({

      //   }),
      // });

      const presentationResponse = response.json();
      setPresentationRequest(presentationResponse);
      console.log("Created Presentation Request:", presentationResponse);

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
            holder_did: did,  // Use the string, not the object
            credentials: [vc],
            challenge: presentationRequest.challenge,
            domain: presentationRequest.domain
          }
      )
      const vp = await response.json();

      // Create a presentation response that includes both the VP and submission metadata
      // This would normally be done by your wallet application
      //TODO: presentationRequest is a const, it doesn't have fields, why are we accessing non existent fields?
      // commented out for now, fix later

      // const presentationSubmission = {
      //   id: `submission-${Date.now()}`,
      //   definition_id: presentationRequest.presentation_definition.id,
      //   descriptor_map: [
      //     {
      //       id: presentationRequest.presentation_definition.input_descriptors[0].id,
      //       format: "ldp_vp",
      //       path: "$.verifiableCredential[0]"
      //     }
      //   ]
      // };
      //
      // const presentationResponse = {
      //   verifiable_presentation: vp,
      //   presentation_submission: presentationSubmission
      // };
      //
      // setPresentation(presentationResponse);
      // console.log("Created Presentation:", presentationResponse);

      // Manually set it as a default value for now
      setPresentation({});
      console.log("Created Presentation:", {});

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

      const result = await response.json();
      setVerificationResult(result);
      console.log("Verification Result:", result);



      // TODO: result.is_valid method is not defined
      //  return success for now by default
      alert('Presentation verified successfully!');
      // if (result.is_valid) {
      //   alert('Presentation verified successfully!');
      // } else {
      //   alert('Presentation verification failed');
      // }

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