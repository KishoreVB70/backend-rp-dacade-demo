use ic_cdk::export_candid;
use std::cell::RefCell;
use ic_canister_sig_creation::extract_raw_root_pk_from_der;
use ic_cdk::{init, post_upgrade};
use candid::{candid_method, CandidType, Principal};
use serde::Deserialize;
use std::collections::HashMap;
use ic_cdk::query;
use ic_verifiable_credentials::{
    issuer_api::{ArgumentValue, CredentialSpec},
    validate_ii_presentation_and_claims, VcFlowSigners,
};

thread_local! {
    static SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ValidateVpRequest {
    pub vp_jwt: String,
    pub effective_vc_subject: Principal,
    pub credential_spec: CredentialSpec,
    pub issuer_origin: String,
}

// Main function
#[query]
#[candid_method]
fn validate_vc_token(vp_jwt: String) -> String {
    SETTINGS.with_borrow(|settings_opt| {
        let settings = settings_opt
            .as_ref()
            .expect("Settings should be initialized");

        // Unique identifier of the caller principal
        let effective_vc_subject = ic_cdk::api::caller();

        // Current system time in nanoseconds, used for validation timestamp
        let current_time_ns: u128 = ic_cdk::api::time() as u128;

        // Define the two signers involved in the Verifiable Credential flow
        let vc_flow_signers = VcFlowSigners {
            // Front settings
            ii_canister_id: settings.ii_canister_id,
            ii_origin: "https://identity.ic0.app/".to_string(),
            // From settings
            issuer_canister_id: settings.issuer_canister_id,
            issuer_origin: "https://dacade.org/".to_string(),
        };

        // Define the expected credential specification for the VP. This spec should match the
        // credential type and argument values in the VP.
        let vc_spec = CredentialSpec {
            credential_type: "Verified TS101 course completion on Dacade".to_string(),
            arguments: Some(HashMap::from([(
                "course".to_string(),
                ArgumentValue::String("TS101".to_string()),
            )])),
        };

        // Validate the VP JWT against the specified signers and credential specifications
        match validate_ii_presentation_and_claims(
            // Input string
            &vp_jwt,
            // User principal
            effective_vc_subject,
            // We're creating, partially from settings
            &vc_flow_signers,
            // Creating
            &vc_spec,
            // Only thing obtained from the settings
            &settings.ic_root_key_raw,
            // Obtaining
            current_time_ns,
        ) {
            Ok(_) => "✅ Success, the credential is valid.".to_string(),
            Err(e) => format!("🛑 Error: {:?}", e),
        }
    })
}


#[query]
#[candid_method]
fn validate_vc_token_allinputs(req: ValidateVpRequest) -> String {
    SETTINGS.with_borrow(|settings_opt| {
        let settings = settings_opt
            .as_ref()
            .expect("Settings should be initialized");

        // Current system time in nanoseconds, used for validation timestamp
        let current_time_ns: u128 = ic_cdk::api::time() as u128;

        // Define the two signers involved in the Verifiable Credential flow
        let vc_flow_signers = VcFlowSigners {
            // Front settings
            ii_canister_id: settings.ii_canister_id,
            ii_origin: "https://identity.ic0.app/".to_string(),
            // From settings
            issuer_canister_id: settings.issuer_canister_id,
            issuer_origin: req.issuer_origin,
        };

        // Define the expected credential specification for the VP. This spec should match the
        // credential type and argument values in the VP.
        let vc_spec = req.credential_spec;
        // Unique identifier of the caller principal
        let effective_vc_subject = req.effective_vc_subject;
        let vp_jwt = req.vp_jwt;

        // Validate the VP JWT against the specified signers and credential specifications
        match validate_ii_presentation_and_claims(
            // Input string
            &vp_jwt,
            // User principal
            effective_vc_subject,
            // We're creating, partially from settings
            &vc_flow_signers,
            // Creating
            &vc_spec,
            // Only thing obtained from the settings
            &settings.ic_root_key_raw,
            // Obtaining
            current_time_ns,
        ) {
            Ok(_) => "✅ Success, the credential is valid.".to_string(),
            Err(e) => format!("🛑 Error: {:?}", e),
        }
    })
}
// init and upgrade logic
#[candid_method(init)]
#[init]
async fn init(settings_input: SettingsInput) {
    save_settings(settings_input);
}

#[post_upgrade]
fn upgrade(settings_input: SettingsInput) {
    save_settings(settings_input);
}

fn save_settings(settings_input: SettingsInput) {
    SETTINGS.with_borrow_mut(|settings| {
        *settings = Some(Settings {
            ic_root_key_raw: extract_raw_root_pk_from_der(&settings_input.ic_root_key_der).unwrap(),
            ii_canister_id: settings_input.ii_canister_id,
            issuer_canister_id: settings_input.issuer_canister_id,
        });
    });
}

// Settings logic
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct SettingsInput {
    pub ic_root_key_der: Vec<u8>,
    pub ii_canister_id: Principal,
    pub issuer_canister_id: Principal,
}

pub struct Settings {
    pub ic_root_key_raw: Vec<u8>,
    pub ii_canister_id: Principal,
    pub issuer_canister_id: Principal,
}


export_candid!();