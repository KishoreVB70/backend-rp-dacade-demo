use ic_cdk::export_candid;
use std::cell::RefCell;
use canister_sig_util::extract_raw_root_pk_from_der;
use ic_cdk::{init, post_upgrade};
use candid::{CandidType, Principal};
use serde::Deserialize;
use std::collections::HashMap;
use ic_cdk::query;
use vc_util::{
    issuer_api::{ArgumentValue, CredentialSpec},
    validate_ii_presentation_and_claims, VcFlowSigners,
};

thread_local! {
    static SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
}

// Main function
#[query]
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
            issuer_origin: "https://ycons-daaaa-aaaal-qja3q-cai.icp0.io/".to_string(),
        };

        // Define the expected credential specification for the VP. This spec should match the
        // credential type and argument values in the VP.
        let vc_spec = CredentialSpec {
            credential_type: "GitcoinPassportScore".to_string(),
            arguments: Some(HashMap::from([(
                "minScore".to_string(),
                ArgumentValue::Int(1),
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

// init and upgrade logic
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