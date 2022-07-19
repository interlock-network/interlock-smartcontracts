/****************************************************************
 * ILOCKsupreme Solana Contract      
 ****************************************************************/

#![allow(non_snake_case)]

use crate::utils::utils::*;

pub enum ContractInstruction {

    ProgramInit {

        bumpGLOBAL: u8,
        seedGLOBAL: Vec<u8>,
    },

    UpdateGlobal {

        updateFlags1: u32,
        updateFlags2: u32,
        values: [u32; VALUES],
    },
    
    CreateUser {

        bumpUSER: u8,
        seedUSER: Vec<u8>,
    },

    FillUser {
        
        amount: u128,
    },

    CreateStake {

        bumpSTAKE: u8,
        seedSTAKE: Vec<u8>,
        amount: u128,
        valence: u8,
    },

    SettleEntity {

        determination: u8,
    },
    
    CloseStake {

        seedENTITY: Vec<u8>,
    },

    CreateEntity {

        bumpSTAKE: u8,
        seedSTAKE: Vec<u8>,
        bumpENTITY: u8,
        seedENTITY: Vec<u8>,
        amount: u128,
        valence: u8,
    },

    SetHunter {

        status: u8,
    },

    CheckEntity {

    },
    
    CloseEntity {

    },

    ClaimEntity {

    },
}


