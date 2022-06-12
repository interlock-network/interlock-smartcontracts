/****************************************************************
 * INTR Solana Contract      
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
    
    CreateRegister {

        bumpREGISTER: u32,
        seedREGISTER: Vec<u8>,
    },
}


