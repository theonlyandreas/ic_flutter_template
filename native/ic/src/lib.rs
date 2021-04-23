use std::{error, fmt, io};
use ring;
use ic_agent::identity::BasicIdentity;
use ic_agent::export::Principal;
use ic_agent::agent::QueryBuilder;
use ic_agent::agent::agent_error::AgentError;


/// A useless Error just for the Demo
#[derive(Copy, Clone, Debug)]
pub struct ICError;
// pub struct MyError {
//     details: String,
// }

impl fmt::Display for ICError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while scrapping this page.")
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

impl From<AgentError> for ICError {
    fn from(_: AgentError) -> Self {
        Self
    }
}

// impl MyError {
//     fn new(msg: &str) -> MyError {
//         MyError{details: msg.to_string()}
//     }
// }

// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"{:?}",self.details)
//     }
// }

// impl error::Error for MyError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }

// impl From<AgentError> for MyError {
//     fn from(err: AgentError) -> Self {
//         MyError::new("Error {} occured with message: {}", err.reject_code, err.reject_message)
//     }
// }

/// Load a page and return its HTML body as a `String`
pub async fn load_page(url: &str) -> Result<String, ICError> {
    Ok(reqwest::get(url).await?.text().await?)
}


// IC Functions
pub async fn query_call(canister_id: &str, method_name: &str) -> String {
    // create random identity for agent
    let rng = ring::rand::SystemRandom::new();
    let key_pair = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
        .expect("Could not generate a key pair.");
    let identity = BasicIdentity::from_key_pair(
        ring::signature::Ed25519KeyPair::from_pkcs8(key_pair.as_ref())
            .expect("Could not read the key pair."),
    );

    // initialize agent and canister_id
    let url = format!("/api/v2/canister/{}/query", canister_id);
    let agent = ic_agent::Agent::builder()
        .with_identity(identity)
        .with_url(url)
        .build()
        .unwrap();
    let canister_id_principal = Principal::from_text(canister_id).unwrap();
    
    // use QueryBuilder to make query call
    let query_builder = QueryBuilder::new(&agent, canister_id_principal, String::from(method_name));
    // Ok(String::from_utf8_lossy(&query_builder.call().await?).to_string())
    let response = query_builder.call().await;
    match response {
        Ok(reply) => return String::from_utf8_lossy(&reply).to_string(),
        Err(AgentError::ReplicaError {
            reject_code,
            reject_message,
        }) => return format!("Error {} occured with message: {}", reject_code, reject_message),
        Err(_) => return "Error occured other than ReplicaError!".to_string(),
    }
}










