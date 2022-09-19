# How to set up local Solana dev node on _most_ OS

```
TODO:
. figure hack to extend to Nix
```

# overview

This guide is intended for operators (users) running Windows, macOS, and linux (Ubuntu, Debian, and CentOS/RHEL).

The purpose of this guide is to install a local Solana development node. 

First we will install the Solana CLI toolset.

As with any blockchain, running a node on mainnet or even testnet is expensive in terms of discspace, and we have to use real sol, or adhere to airdrop limits, so we will focus on a local development node, which allows a lot more freedom.

To be clear, to 'install a local node', is to create a brand new Solana blockchain on your computer.

After we install CLI tools we will setup a new wallet with keypair.

After we get set up, we will fire that bad-boy up.

Finally, we will do some stuff to flex a little.

**Thus the order of operations will go like this:**

1. get solana CLI

2. configure and setup

3. run new blockchain with local devnode

4. do some stuff with CLI tools

5. outro






# 1. get solana CLI
<details>
<summary markdown="span">expand</summary>
&emsp;

First we need solana.

Choose your operating system:

<details>
<summary>&emsp;macOS, Ubuntu, Debian, CentOS/RHEL</summary>
&emsp;

It's easy. Run the script from the client url request like so:

```
sh -c "$(curl -sSfL https://release.solana.com/v1.9.13/install)"
```

And export to PATH:
```
export PATH="/home/username/.local/share/solana/install/active_release/bin:$PATH"
```
Run
```
source .profile
```
or restart for PATH update to take effect.

&emsp;
</details>


<details>
<summary>&emsp;Windows</summary>
&emsp;

Windows is difficult. Good luck:

It is best to avoid DLL hell and instead run solana within a Windows Linux susbsystem environment.

From the command prompt, as administrator, check to see if `wsl` is installed:
```
wsl
```

If the command is not found, install `wsl` by running:
```
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

If `wsl` is installed, but no Linux systems are installed, then `wsl` will return the `wsl --help` menu.

If Linux systems do exist, then running `wsl` will open a bash terminal in that subsystem default instance.

Chances are though, you don't have a Linux subsystem installed, so in any case after you install `wsl`, [then download Kali Linux from the Microsoft store, here](https://www.microsoft.com/en-us/p/kali-linux/9pkr34tncv07?rtc=1#activetab=pivot:overviewtab). Kali Linux is lighter weight, and arguably more badass than the few alternatives in the Microsoft store. Do the right thing.

After download, Kali Linux will automatically boot to the open command prompt terminal. If it doesn't, or if you need to return, run it manually:
```
wsl -d kali-linux
```

Once you are in, `cd` to user home directory (`wsl` by default creates a Windows mount to the Kali OS, awkward).

Now we can proceed to install Solana CLI. Part of the reason why Kali is better in this case is that we don't need to fiddle with getting `curl`. If I am wrong, then you need to `sudo apt install curl`.

Get Solana CLI:
```
sh -c "$(curl -sSfL https://release.solana.com/v1.9.13/install)"
```

And export to PATH:
```
export PATH="/home/username/.local/share/solana/install/active_release/bin:$PATH"
```
Run
```
source .profile
```
or restart for PATH update to take effect.

&emsp;
</details>

<details>
<summary>&emsp;Nix</summary>
&emsp;

Unfortunately NixOS isn't yet supported...something about issues with the BPF llvm toolchain stuff.

To run a Solana dev node on Nix, create a Docker container with your favorite image supported by this guide.

</details>

&emsp;

------------------

Verify the install by running `solana --version` in a terminal, which should return something like this:
```
blairmunroakusa$ solana --version
 solana-cli 1.9.5 (src:39a4cc95; feat:3125401026)
```

</details>
&emsp;






# 2. configure and setup
<details>
<summary>expand</summary>
&emsp;

Solana is easier to configure a new local blockchain. This local node is a _local cluster_ and we call it a _test validator_.

**This section is the same for all systems.**

First we need to configure the CLI url to the localhost:
```
solana config set --url localhost
```

Verify the config change with `get`, which should return something like this:
```
blairmunroakusa$ solana config get
 Config File: /Users/blairmunroakusa/.config/solana/cli/config.yml
 RPC URL: http://localhost:8899 
 WebSocket URL: ws://localhost:8900/ (computed)
 Keypair Path: /Users/blairmunroakusa/.config/solana/id.json 
 Commitment: confirmed
```

Next you will notice from the `get` above that we have a `Keypair Path` we need to tend to.

We need to generate a keypair for your main account on this particular node/cluster. This account is **_owned_** by the system program, but you have the **_authority_** over the account provided by your possession of the private key used to generate the public key account address. In Solana, all accounts are owned by **_programs_**. A program is equivalent to a **_smart contract_** in Ethereum. By default, all accounts are owned by the **_system program_**. Think of the system program like the master smart contract.

This main account we are creating is where we airdrop fake SOL, so we can do stuff and other stuff. The command will ask you for a password, provide you with a pubkey, and provide you with a mnemonic seed phrase for recovery. _YOUR WALLET IS THE ID.JSON FILE IN YOUR .CONFIG DIRECTORY._

Run:
```
solana-keygen new
```

Now the id.json file in the Solana .config directory will contain the pub/pri key for your 'machine's id'. You can use this keypair for all future testnet/localnet dev work you do. You cannot use this keypair with real SOL. For that, you need a proper account on mainnet-beta.

The prikey is stored in the id.json, so it will be convenient to write the pubkey down somewhere easy for future reference.

To get your pubkey in case you need it, run:
```
solana address
```

Ok cool. Ready to rock.

</details>
&emsp;







# 3. run new blockchain with local devnode
<details>
<summary>expand</summary>
&emsp;

**Running the new blockchain is (relatively) easy:**

Running the node is as easy as:
```
solana-test-validator
```

But not so fast, Windows user. First we need to install `bzip2`:
```
sudo apt install bzip2
```
OK

If you want the 'streaming matrix' effect, run `solana-test-validator --log`.

If you want, you can turn on logs in a separate process. These logs contain information about transaction success, and msg! macros from Solana programs.
```
solana logs
```

After running the validator, you will notice that Solana doesn't provide a streaming record of blocks. It just kind of keeps track of things and should look something like this:
```
blairmunroakusa$ solana-test-validator
 Ledger location: test-ledger
 Log: test-ledger/validator.log
 ⠈ Initializing...
 Identity: 9pbaMzQKqhiPetty2GfTh9L2nP9zEpe1ViaNPSfidwFM
 Genesis Hash: DQDBtE2bct1UE3KEdpJffaK7SLsAESFG4NLpBuytuU3c
 Version: 1.9.5
 Shred Version: 65326
 Gossip Address: 127.0.0.1:1024
 TPU Address: 127.0.0.1:1027
 JSON RPC URL: http://127.0.0.1:8899
 ⠚ 00:01:35 | Processed Slot: 197 | Confirmed Slot: 197 ...
```

</details>
&emsp;






# 4. do some stuff with CLI tools
<details>
<summary>expand</summary>
&emsp;

**This section is the same for all systems:**

Okay, let's do some stuff.

After you have launched a new test-validator node, open a new terminal session, leaving the blockchain process running (that or just run the process in the background instead).

Before we can really do anything, we need some money. Time for a SOL

Chances are your account was prefunded with a large amount of SOL, but we will go through the motions and airdrop anyways.

Get your address/pubkey:
```
solana address
```

Or get your general account info:
```
solana account $(solana address)
```

Let's airdrop 1000 SOL to your account:
```
solana airdrop --url localhost 1000 $(solana address)
```

Cool. Your balance should have increased by 1000 SOL.

It should be noted that most chain manipulation is done with the web3 API. The CLI is pretty clunky and limited. There is no fancy javascript console like with `geth`, so we are limited in how much we can do over CLI without creating additional accounts first via the web3 API.



</details>
&emsp;






# 5. outro

And that's that. If you haven't gotten lost, you just configured a private Solana blockchain and got it running with a single local dev node on your machine.

In a future article, we will review how to set up additional nodes on a private Solana dev blockchain.

# sequel, tidbits

. [visit official Solana docs here](https://docs.solana.com)

. [web3 API docs](https://docs.solana.com/developing/clients/javascript-api)

. [help with WSL install](https://docs.microsoft.com/en-us/windows/wsl/install)

. All OS implementations were tested; if you can't make this work you're probably doing something wrong.

. If you want to run this in Docker via this guide, choose an image that this guide supports.
