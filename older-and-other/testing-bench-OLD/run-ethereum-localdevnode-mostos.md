# How to set up local Ethereum dev node on _most_ OS

```
TODO:
. extend for centOS / RHEL
```

# overview

This guide is intended for operators (users) running Windows, macOS, and linux (Ubuntu, Debian, and Nix).

To use this guide, you must know how the terminal on your machine.

The purpose of this guide is to install a bare-bones local Ethereum development node. Barebones means that we will not be using a framework like Hardhat or Truffle.

First we will install geth (geth for 'Go Ethereum') which is the official Ethereum client CLI suite. Geth is cool because it comes with a javascript console for interacting with the chain via the web3.js and other APIs.

Running a full node on mainnet is expensive (need ~500GB free drive space), and we have to use real eth, so we will focus on a local development node, which doesn't require so much resource.

To be clear, to 'install a local node', is to create a brand new Ethereum blockchain on your computer.

After we install geth we will setup a new blockchain.

After we setup the new blockchain, we will fire that bad-boy up.

Finally, we will do some stuff to flex a little.

**Thus the order of operations will go like this:**

1. get geth

2. setup new blockchain

3. run new blockchain with local devnode

4. connect to node with client and do some stuff

5. outro






# 1. get geth
<details>
<summary markdown="span">expand</summary>
&emsp;

First we need geth.

Choose your operating system:

<details>
<summary>&emsp;macOS</summary>
&emsp;

[If you don't have homebrew, you first need to install it by following the guide at this link.](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-homebrew-on-macos)

Use homebrew to install geth:
```
brew tap ethereum/ethereum
brew install ethereum
```

&emsp;
</details>


<details>
<summary>&emsp;Windows</summary>
&emsp;

Go here, and download the pre-compiled binary executable then install:

[https://geth.ethereum.org/downloads/](https://geth.ethereum.org/downloads/)

Geth will now be available from the command prompt.

&emsp;
</details>


<details>
<summary>&emsp;Ubuntu</summary>
&emsp;

Use `apt-get` to install geth:
```
sudo add-apt-repository -y ppa:ethereum/ethereum
sudo apt-get update
sudo apt-get install ethereum
```

&emsp;
</details>


<details>
<summary>&emsp;Debian</summary>
&emsp;

First create a new apt source list file:
```
sudo touch /etc/apt/sources.list.d/ethereum.list
```

Then add these two lines to the file:
```
deb http://ppa.launchpad.net/ethereum/ethereum/ubuntu bionic main 
deb-src http://ppa.launchpad.net/ethereum/ethereum/ubuntu bionic main
```

Now import this GPG key for the PPA:
```
sudo apt-key adv --keyserver keyserver.ubuntu.com  --recv-keys 2A518C819BE37D2C2031944D1C52189C923F6CA9
```

Then update and install:
```
sudo apt update
sudo apt install ethereum
```

&emsp;
</details>

<details>
<summary>&emsp;Nix</summary>
&emsp;

From your terminal, install the go-ethereum package by running:
```
nix-env -iA nixos.go-ethereum
```

And simple as that, this should be enough to have access to geth and other utilities.

</details>

&emsp;

------------------

Verify the install by running `geth version` in a terminal, which should return something like this:
```
blairmunroakusa$ geth version
 Geth
 Version: 1.10.16-stable
 Architecture: amd64
 Go Version: go1.17.6
 Operating System: darwin
 GOPATH=
 GOROOT=go
```

And this is geth. With this tool installed, we can not only manipulate a local dev node, but we can also connect and interact with the Ethereum mainnet, accounts, smartcontracts, etc.

</details>
&emsp;






# 2. setup new blockchain
<details>
<summary>expand</summary>
&emsp;

The first thing we need to do is set up the genesis block, which in general is the first block on any blockchain.

We will be using a tool (CLI wizard) called puppeth, which typically bundles with geth.

**This section is the same for all systems.**

Create a new blockchain space on your machine, create a new subspace for the genesis node, then run `puppeth` from the blockchain space:
```
mkdir interlockdev-ethereum
cd interlockdev-ethereum
mkdir devnode1
puppeth
```

This will take us through the following setup prompt menu to configure a new genesis block.

Choose a network name:
```
blairmunroakusa$ puppeth
.
. <intro placcard here>
.

Please specify a network name to administer (no spaces, hyphens or capital letters please)
> interlockdev
```

Configure new genesis:
```
What would you like to do? (default = stats)
 1. Show network stats
 2. Configure new genesis
 3. Track new remote server
 4. Deploy network components
> 2
```

Create ours from scratch:
```
What would you like to do? (default = create)
 1. Create new genesis from scratch
 2. Import already existing genesis
> 1
```

>_As of writing, 17Mar22, Ethereum mainnet is running a proof-of-work consensus algorithm, with a transition to proof-of-stake consensus sometime Q1-Q2, 2022._

We will create a proof-of-work chain.

Choose proof-of-work:
```
Which consensus engine to use? (default = clique)
 1. Ethash - proof-of-work
 2. Clique - proof-of-authority
> 1
```

Disregard prefunding account:
```
Which accounts should be pre-funded? (advisable at least one)
> 0x
(not pre-funding any)
```

>_Note, accounts in ethereum are 40 digit hexadecimal numbers._

Do what is advisable:
```
Should the precompile-addresses (0x1 .. 0xff) be pre-funded with 1 wei? (advisable yes)
> yes
```

Specify default network ID:
```
Specify your chain/network ID if you want an explicit one (default = random)
> 1234
```

Choose manage genesis we just created above:
```
What would you like to do? (default = stats)
 1. Show network stats
 2. Manage existing genesis
 3. Track new remote server
 4. Deploy network components
> 2
```

Export config for new genesis block:
```
 1. Modify existing configurations
 2. Export genesis configurations
 3. Remove genesis configuration
> 2
```

And finally, save to `interlockdev-ethereum` directory:
```
Which folder to save the genesis specs into? (default = current)
>
(no selection is default)
```

Okay cool. Our blockchain genesis block is setup. Exit out of `puppeth`. Your `interlockdev-ethereum` directory should look like the following:
```
 devnode1
 interlockdev-aleth.json
 interlockdev-harmony.json
 interlockdev-parity.json
 interlockdev.json
```


</details>
&emsp;







# 3. run new blockchain with local devnode
<details>
<summary>expand</summary>
&emsp;

**Running the new blockchain is essentially the same for all systems:**

We start by initializing a single node. This `init` command will create a `keystore` directory for private keys, and a `geth` (containing `chaindata`) directory for general blockchain and node data. We will be setting up `devnode1` as the primary node that runs the blockchain.

From `interlockdev-ethereum` run:
```
geth --datadir devnode1 init interlockdev.json
```

Now we need to create an account that represents this node. This is the default etherbase.

>_The etherbase or coinbase is the account that mining proceeds are deposited in._

From `interlockdev-ethereum` run:
```
geth --datadir devnode1 account new
```

Create a password as prompted (remember it), and copy the account address for future reference. Mine is `0x2402c3fe2f60e3cdc951c61450dd0c80aa0baeb5`. Save the password in a file within `devnode1` directory:

For macOS, Ubuntu, Debian, Nix:
```
echo mypastedpassword > devnode1/password.sec
```

For Windows:
```
echo mypastedpassword> devnode1\password.sec
```

Your `devnode1` directory should now contain the following:
```
 geth
 keystore
 password.sec
 ```

OK

It's time to fire up the blockchain, originating at `devnode1`.

From `interlockdev-ethereum` run the following on macOS, Ubuntu, Debian, Nix:
```
geth --networkid 1234 --mine --miner.threads 1 --http.api web3,eth,personal,net --unlock 0 --nodiscover --datadir devnode1 --password devnode1/password.sec --ipcpath devnode1/geth.ipc
```

Or on Windows run:
```
geth --networkid 1234 --mine --miner.threads 1 --http.api web3,eth,personal,net --unlock 0 --nodiscover --datadir devnode1 --password devnode1\password.sec --ipcpath devnode1\geth.ipc
```


> _Default RPC port is 8545. Default listening port is 30303. Usual stuff._

At this point you may see a streaming list of
```
INFO [date] Generating DAG in progress
```

which will eventually transition to
```
INFO [date] 🔨 mined potential block
INFO [date] Commit new sealing work 
```

This means your machine is maintaining an Ethereum blockchain and the node is mining to verify blocks and any transactions which might occur within (none so far). The account we made earlier is our primary account, or our _etherbase_ or _coinbase_ which is where this blockchain will deposit ether earned by mining a block. We can specify a different etherbase at any time.

If you need to stop the blockchain process, you can always restart it later and the process will resume at it's final state.

Now it's time to...
</details>
&emsp;






# 4. connect to node with client and do some stuff
<details>
<summary>expand</summary>
&emsp;

**This section is essentially the same for all systems:**

Open a new terminal session, leaving the blockchain process running (that or just run the process in the background instead). Now we want to connect to the node by firing up a javascript console. This is easy. Run:

For macOS, Ubuntu, Debian, Nix:
```
geth attach devnode1/geth.ipc
```

Windows:
```
geth attach ipc:\\.\pipe\devnode1\geth.ipc
```

Now we can fire off web3, eth, and personal commands to interact with the blockchain.

First, check to see which accounts are active. It should only be the etherbase:
```
> eth.accounts
["0x2402c3fe2f60e3cdc951c61450dd0c80aa0baeb5"]
```

Yup, that was my etherbase account.

Now let's get the etherbase balance:
```
> web3.fromWei(eth.getBalance(eth.accounts[0]), "ether")
126
```

This command gets the balance of account 0 in wei, and converts it into ether. There are  a lot of wei in one ether, so it is nice to convert things to ether first.

> There are 1,000,000,000,000,000,000 (10^18) wei in one ether.

Really quick, let's create an account now and transfer some ether to it.
```
> personal.newAccount()
```

Choose a password, now we have
```
> eth.accounts
["0x2402c3fe2f60e3cdc951c61450dd0c80aa0baeb5", "0x83c257547034b056b23b0807f967045d2b59af4b"]
```

but unfortunately something sad:
```
> web3.fromWei(eth.getBalance(eth.accounts[1]), "ether")
0
```

So let's finish up the hands-on by creating a transfer transaction:
```
> eth.sendTransaction({from: eth.accounts[0], to: eth.accounts[1], value: 200000000000000})
"0x53c3acf3333cd4a35d3b7752746c449974bc96b84729dca751b4a2304f8c4b88"
> web3.fromWei(eth.getBalance(eth.accounts[1]), "ether")
0.0002
```

It's not much, but sometimes 0.0002 eth between friends really is better than nothing.

The long hex number is the transaction hash.
</details>
&emsp;






# 5. outro

And that's that. If you haven't gotten lost, you just configured a private Ethereum blockchain and got it running with a single local dev node on your machine.

In a future article, we will review how to set up additional nodes on a private Ethereum dev blockchain.

# sequel, tidbits

. [visit official go-ethereum docs here](https://geth.ethereum.org)

. [web3 Ethereum API docs](https://web3js.readthedocs.io/en/v1.7.1/)

. All OS implementations were tested; if you can't make this work you're probably doing something wrong.

. Drop `--nodiscover` flag from blockchain startup if you want your chain available to mystery network peers.

. And disregarding the nontrivial task of rebranding, literally that is how easy it is to create a new Ethereum blockchain. Call yours what you will. My local chain is called **hyperbloX**, with a capital X.

. If you want to run this in Docker via this guide, choose an image that this guide supports.
