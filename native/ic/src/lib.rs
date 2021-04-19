use std::{error, fmt, io};
// use ring;
// use ic_agent::BasicIdentity;
// use ic_agent::export::Principal;
// use ic_agent::agent::QueryBuilder;
// use ic_agent::agent::agent_error::AgentError;


/// A useless Error just for the Demo
#[derive(Copy, Clone, Debug)]
pub struct ICError;

impl fmt::Display for ICError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error While ICping this page.")
    }
}

impl error::Error for ICError {}

impl From<reqwest::Error> for ICError {
    fn from(_: reqwest::Error) -> Self {
        Self
    }
}

impl From<io::Error> for ICError {
    fn from(_: io::Error) -> Self {
        Self
    }
}

/// Load a page and return its HTML body as a `String`
pub async fn load_page(url: &str) -> Result<String, ICError> {
    Ok(reqwest::get(url).await?.text().await?)
}


// IC Functions
// pub async fn query_call(canister_id: &str, method_name: &str) -> Result<Vec<u8>, AgentError> {
//     // create random identity for agent
//     let rng = ring::rand::SystemRandom::new();
//     let key_pair = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
//         .expect("Could not generate a key pair.");
//     let identity = BasicIdentity::from_key_pair(
//         ring::signature::Ed25519KeyPair::from_pkcs8(key_pair.as_ref())
//             .expect("Could not read the key pair."),
//     );

//     // initialize agent and canister_id
//     let agent = ic_agent::Agent::builder()
//         .with_identity(identity)
//         .build()
//         .unwrap();
//     let canister_id_principal = Principal::from_str(&canister_id).unwrap()
    
//     // use QueryBuilder to make query call
//     let queryBuilder = QueryBuilder::new(&agent, &canister_id_principal, &method_name);
//     queryBuilder.call().await   
// }










