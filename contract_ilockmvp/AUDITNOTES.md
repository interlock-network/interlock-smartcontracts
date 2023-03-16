# Kudelski Audit Notes -- ilockmvp

This document is intended to serve as a communication tool between Interlock and Kudelski for the ongoing smartcontract audit and review.

Please feel free to post questions or comments as you audit, make note in telegram, and I will do my best to answer/address as they come up.

To kick things off, at the end of this document are a handful of preliminary thoughts and notes that I hope will help you moving forward with this audit.

# Questions

#### Please ask a question as a L4 header


# Comments

#### Please leave a comment as a L4 header


# Preliminary Thoughts and Notes

- The checked math is due to need to turn off compiler checks in order to use floor division that does not trap contract.
- I was probably less discriminate than need be with the checked math, as there are many cases where over/underflow is impossible, if PSP22 standard spec holds.
- In order to enforce owner-can't-transfer, I took a lot of the transfer functions out of the rewards and distribution functions. This made it so I could implement those same e2e tests wih simple unit tests, but alas, I ran out of time.
- The application port/socket formalism will be the most challenging aspect of this contract, conceptually. When you do make the attempt, do study the application and uanft contracts/readmes in tandem to provide more insight.
- This is a mono contract, deliberately. I saw no need to go on breaking things into traits and implementations with this and that file to boot. The goal is to keep the code straight forward and as simple as possible.
- Along those lines, the code is quite wet. I believe the current state is simplest.
- The TESTING_increment_month function is for us to explore the contract's behavior on live blockchain. This is to be removed prior to TGE, but I wanted to leave it for your convenience, should you choose to tinker.
- Tests are broken out into separate files for unit and end-to-end to keep contract filesize down.
