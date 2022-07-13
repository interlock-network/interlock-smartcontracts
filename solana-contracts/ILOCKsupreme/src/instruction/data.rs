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

        updateFlags: u32,
        values: [u32; VALUES],
    },
    
    CreateAccount {

        bumpACCOUNT: u8,
        seedACCOUNT: Vec<u8>,
    },

    FillAccount {

    },

    CreateStake {

        bumpSTAKE: u8,
        seedSTAKE: Vec<u8>,
        amount: u64,
    },

    SettleEntity {

        determination: u8,
    },
    
    CloseStake {

    },
}


