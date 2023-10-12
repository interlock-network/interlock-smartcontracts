# Interlock Network Blockchain Method of Procedure #0:

## Method of Procedure Overview and Template

Metadata:
```
Creation date: 10/10/2023
Revision date: 10/10/2023
Authors:
blairmunroakusa

```
TODO:
```
- 
```

### Purpose and Scope:

#### What

A Method of Procedure (MOP) provides detailed, step-by-step instructions ensuring consistency and accuracy in executing specific tasks, minimizing the chance for errors and operational disruptions.

#### Why

In the realm of web3 and blockchain, where transactions are immutable and security vulnerabilities can have irreversible consequences, a well-defined MOP ensures that procedures are carried out consistently, safeguarding the integrity, security, and reliability of the decentralized system.

#### Additional Information

This document and outlined procedure are with simplified industry best-practices in mind. Much of this is drawn from telecommunications network implementation, management, maintenance, and outage response.

### Roles and Expected Outcomes:

Your role in the context of you reading this document is anybody within Interlock Network or the community who needs to create a method of procedure for some task or routine.

After following these steps, you should have a MOP that your target audience is capable of following without further guidance or assistance. Because MOPs are living documents, soliciting for feedback from MOP users will in most cases lead revision improvements over time. Your MOP should look like this MOP.

### Resources and Safety:

In the context of this MOP #0, safety is not such a concern, but all concerning issues or flaws found within a MOP must be integrated into a MOP revision as soon as possible, especially if the issue or flaw caused damage or harm.

As for resources, this approach to MOP and SOP management is meant to stay simple, as a singled directory containing all available operating procedures and methods of procedure within the single Interlock Network smartcontract monorepo.

### Detailed Instructions:

To create a new MOP, perform the following. Template in reference is found at the bottom of this MOP in the _References_ section:

#### 1. clone template

Copy this template document -- `./mop0.methodOfProcedureTemplate.md` -- to a new mop file in this format: `bcmop<<number>>.<<mop title in camel case>>.md`. Edit the file by following remaining steps.

#### 2. populate Purpose and Scope section

Define the purpose and scope of the MOP. That is, specify the high level general _what_ that the MOP addresses, define the corresponding _why_ to justify it, ideally in terms of a SOP. Consider this MOP's _Purpose_ section above as an example. Fill in the _Purpose and Scope_ section of the template. If additional information is justified, add that to the _Additional Information_ subsection.

#### 3. populate Roles and Expected Outcomes section

Define the relevant roles and the expected outcome of the MOP in the context of the purpose and scope. A role is anybody who might use the MOP, which is the target audience. For example, this may be "CEO or advisor successfully launch and initialized ILOCK rewards contract on the Aleph Zero blockchain". The expected outcome is a _what_. Fill in the _Expected Outcomes_ section of the template.

#### 4. populate the Review and Reference section

If there is any outcome review or MOP execution approval needed, specify that here. Collect any resources that are immediately relevant to carrying out the MOP successfully. Fill out the _Review and References_ section of the template.

#### 5. create detailed steps and instructions

Write out the _Steps_ needed to achieve the expected outcome, and provide enough detail that the target audience will be able to successfully and reliably carry out the MOP.

#### 6.

Add the creation date and revision date.

### References

Here is the MOP template that this repository adheres to:

```
# Interlock Network Blockchain Method of Procedure #<<MOP number>>:

## <<MOP title>>

##### Metadata:
```
Creation date: <<creation date MM/DD/YYYY>>
Revision date: <<revision date MM/DD/YYYY>>
```
##### TODO:
```
<<Stuff that needs todo attention here, as bullet list>>
```

### Purpose and Scope:

#### What

<<Description of what this MOP is about goes here.>>

#### Why

<<Description of why this MOP is necessary and important goes here.>>

#### Additional Information

<<Add any additional relevant information here.>>

### Roles and Expected Outcomes:

<<Add info about MOP roles and what to expect here.>>

### Resources and Safety:

<<Mention resources MOP user will need and also the safety risks they will face and how to mitigate.>>

### Detailed Instructions:

<<Enumerated list of steps like:
#### 1. <<optional step title>>
#### 2.
etc>>

### References

<<Related documents or resources go here.>>
```
