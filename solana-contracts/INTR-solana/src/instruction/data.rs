/****************************************************************
 * Solana program template         
 ****************************************************************/

#![allow(non_snake_case)]

// example enum declaration for instructions and instruction_data variables
//
// Vec<8> type is because the serialized string may be between 0 and 32 B
// (size unknown at compile time)

pub enum ContractInstruction {

    ProgramInit {

        bumpGLOBAL: u8,
        seedGLOBAL: Vec<u8>,
    },

    UpdateGlobal {

        dataNumberA: u8,
        dataStringA: Vec<u8>,
    },

    InstructionThree {

        dataNumberC: u32,
    },
}


