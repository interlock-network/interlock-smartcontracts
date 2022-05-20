/****************************************************************
 * INTR Solana Contract      
 ****************************************************************/

#![allow(non_snake_case)]

pub enum ContractInstruction {

    ProgramInit {

        bumpGLOBAL: u8,
        seedGLOBAL: Vec<u8>,
    },

    UpdateGlobal {

    },

    InstructionThree {

        dataNumberC: u32,
    },
}


