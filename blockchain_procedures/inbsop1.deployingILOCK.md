# Interlock Network Blockchain Standard Operating Procedure #1:

## Deploying ILOCK

##### Metadata:
```
Creation date: 10/10/2023
Revision date: 10/10/2023
```
##### SOP TODO:
```
- submit to other Interlock employees for review
- break original detailed instructions into several MOPs
```

### Purpose and Scope:

#### What

This SOP outlines the steps to deploy an ILOCK solidity contract on Arbitrum and EVM blockchains in general. It details the setup requirements, stakeholder data preparation, contract deployment, validation, and final sanity checks. This assumes the case where the deployment is an initial TGE, and the vesting schedule must be implemented.

#### Why

To standardize the deployment process ensuring a consistent and secure establishment of the ILOCK contract on the Arbitrum platform and future EVM deployments.

#### Additional Information

It is essential to be thorough and precise in each step, as some actions, especially around stake registration, can be irreversible. However, there are ways to 'start over' if absolutely necessary (though this be potentially dangerous from a contract phishing perspective).

### Procedure and Roles:

This SOP primarily designates roles to the contract deployer and highlights the involvement of multisig signatories and stakeholders. By the end of this process, the contract should be live, validated, and tokens should be distributed as intended.

This process and role selection is critical from a security perspective, because the owner of this account and the owner of multisig private keys have absolute control over the sensitive stakeholder stake data that needs to be entered into the contract.

The roles are:

- blockchain engineer, to set up launch environment
- executive operator, to execute the launch
- stakeholder observers to witness the launch

### Reference MOPs:

Due to the relative complexity of an initial ILOCK solidity contract launch, this SOP will involve a few different MOPs:

1 - prelaunch preparation
2 - setup organization's Safe multisig safe (wallet)
3 - setup deployment environment
4 - prelaunch checklist
5 - execute deployment
6 - prepare, input, and verify stakeholder data
7 - execute and verify TGE

### Resources and Communication:

##### Resources Needed:
- Account access to [https://safe.global](https://safe.global)
- Required amount of $ARB or other $EVM other coin
- Contract workspace within `contract_ilockmvp_sol`
- Stakeholder data in specified formats

##### Communication Channels:
- Email for stake claim receipts and distribution method
- Regular updates and checkpoints with executive stakeholders
- Direct access to multisig signatories

### Review and Update:

##### This SOP should be reviewed:
- After initial ILOCK TGE
- After succeeding later ILOCK deployments

##### Process for making updates:
1. Make it happen.

### References

1. [Arbitrum Documentation](https://developer.offchainlabs.com/docs/)
2. [Hardhat Guide](https://hardhat.org/guides/)
3. [safe.global User Manual](https://safe.global/manual/)
4. Interlock Network Smart Contracts repository (for deployment data and stake information)
