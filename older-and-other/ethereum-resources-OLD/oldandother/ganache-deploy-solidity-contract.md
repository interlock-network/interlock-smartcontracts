## Testing Solidity contracts locally with Ganache

This is a guide to lead you through the steps to setup a Ganache blockchain simulator, to deploy and begin basic local functionality testing for Solidity smart contracts.

This setup guide presumes a unix-based operating system such as Linux or MacOS. It also presumes the knowledge it takes to get a Solidity contract to compile.

### 1) Setup Ganache CLI

Ganache is an Ethereum client oriented about node.js. Ganache is ideal for basic testing because it is very fast, and **not** resource intensive.

**To begin, make sure you have `npm`:**

To get `npm` with a Mac, use homebrew,
```
brew install npm
```

To get `npm` with a Linux, use your distribution's default package manager,
```
sudo apt install npm
```

**Next, make sure you have `node.js`:**

To get `node.js`, [make that happen here.](https://nodejs.org/en/download/)

**Now you have what you need to get Ganache:**

```
npm install -g ganache-cli
```

### 2) Run Ganache CLI

We want to run Ganache on the default 8545 port:
```
ganache-cli -p 8545
```

Ok, good to go. You will see a list of pregenerated accounts for your tinker leisure.

### 3) Compile and prepare contract for deployment

First, make sure you have `solc` (solidity compiler) and `web3` API:

```
npm install -g web3
```
and 
```
npm install -g solc
```
Now we take our contract (Remix makes for a quick way to get a contract compiling) and compile locally.

```
solcjs --bin --abi MyConstract.sol
```
When this successfully compiles, we then need to save the bin and abi to variables for future use.

Start a new `node` session:
```
node
```
Now stringify the bytecode:
```
> bytecode = fs.readFileSync('MyContract_sol_MyContract.bin').toString()
```
And stringify the ABI:
```
> abi = JSON.parse(fs.readFileSync('MyContract_sol_MyContract.abi').toString())
```

### 4) Deploy contract

With the Ganache client running in a different terminal session, create a new connection object to the client:
```
> Web3 = require('web3')
> web3 = new Web3('http://localhost:8545')
```
Now create a contract object that we can deploy next:
```
> MyConract = new web3.eth.Contract(abi)
```
Which now, we can deploy. For the `from` address, choose one of the pregenerated addresses provided by the Ganache client.
```
> MyContract.deploy({data:bytecode,arguments:[args]}).send({from:'0x1234567890123456789012345678901234567890,gas:5000000,gasPrice:web3.utils.toWei('00000005','ether')}).then((newContractInstance)=>{MyContract.options.address=newContractInstance.options.address})```
After deploying, you will see the transaction block in the Ganache console.

NB! If your contract constructor needs arguments passed to it, include those as an array in the `arguments` field above. If NOT, then remove the `arguments` field.
