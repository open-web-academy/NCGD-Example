use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
    PromiseResult, Gas, require, serde_json::json
};


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Puntuacion {
    // token_id : String,
    id_jugador : String,
    puntuacion : String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,
    pub puntuaciones: HashMap<String,Puntuacion>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init_contract(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id
        )
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            owner_id,
            puntuaciones:HashMap::new()
        };

        //return the Contract object
        this
    }

    // Obtener puntuación del jugador
    pub fn obtener_puntuacion(&self) -> Puntuacion {
        let jugador = env::signer_account_id();

        let puntuacion = self.puntuaciones.get(&jugador.to_string());
        
        if puntuacion.is_none() {
            env::panic_str("Esta cuenta no tiene puntuación");
        }

        let info = puntuacion.unwrap();

        let info_puntuacion = Puntuacion {
            id_jugador : info.id_jugador.to_string(),
            puntuacion : info.puntuacion.to_string()
        };

        env::log(
            json!(info_puntuacion.clone())
            .to_string()
            .as_bytes(),
        );

        info_puntuacion
    }

    // Guardar puntuación de jugador
    pub fn guardar_puntuacion(&mut self, puntuacion: u64) -> Puntuacion {
        let jugador = env::signer_account_id();

        let info_puntuacion = Puntuacion {
            id_jugador : jugador.to_string(),
            puntuacion : puntuacion.to_string()
        };

        self.puntuaciones.insert(jugador.clone().to_string(),info_puntuacion.clone());

        env::log(
            json!(info_puntuacion.clone())
            .to_string()
            .as_bytes(),
        );

        info_puntuacion
    }

    // Obtener todas las puntuaciones de los jugadores
    pub fn obtener_puntuaciones(&self) {
        env::log(
            json!(self.puntuaciones.clone())
            .to_string()
            .as_bytes(),
        );
    }


}