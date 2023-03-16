# Kudelski Audit Notes -- uanft

This document is intended to serve as a communication tool between Interlock and Kudelski for the ongoing smartcontract audit and review.

Please feel free to post questions or comments as you audit, make note in telegram, and I will do my best to answer/address as they come up.

To kick things off, at the end of this document are a handful of preliminary thoughts and notes that I hope will help you moving forward with this audit.

# Questions

#### Please ask a question as a L4 header


# Comments

#### Please leave a comment as a L4 header


# Preliminary Thoughts and Notes

- The checked math is due to need to turn off compiler checks in order to use floor division that does not trap contract.
- I am treating Art Zero codebits akin to openbrush. They have been audited, but I have not invested a great deal of energy into scouring their bits for soundness. This is a risk exposure. I am not so interested in that their bits work as they intend, for they are more the interface to their marketplace. I just want to be sure their bits don't somehow compromise the soundness of the overall contract and PSP34 standard.
- In terms of the Art Zero bits, we have been instructed to just 'use them because they work for them'...they are extracted from their template, largely copypasta.
- Check out the demoapp if you want to get a better feel for the uanft lifecycle.
- This nft is a reference implementation for the application port/socket contract template, and it demonstrates working interaction with a port on the ILOCK PSP22 contract.
