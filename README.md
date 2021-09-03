<p align="center">
  <a href="https://cropper.finance">
    <img alt="Cropper" src="https://cropper.finance/assets/Component2.png" width="150" />
  </a>
</p>

# Permissionless Yield Farming on Solana

The project comprises of:

* An on-chain yield farming program

* A “B2B” “B” user must be able to create a “farm”, which is broken down into several parts: (AMM ID (taking the pair of tokens), a reward rate (currently set at 1% of the reward pool / week), a sequestration of reward for the “farmers” (in B2B_token)).

* A summary front aspect including the configuration elements will be necessary in order to carry out tests

* It will only be emptied for the distribution of rewards;

* (time 2) At farm creation, if the LP token is not a B2B_token-CRP composition, user B will be subject to an additional fee of 5000 USDC;

* N “C” users must be able to add liquidity to this farm and receive rewards according to the reward rate and the proportion they represent in the farm (Ex: C1 is the owner of 50% of the liquidity of the farm, in 1 week he will receive 50% x 1% x reward vault, C2 having 10% will only receive 10% x 1% x reward vault);

* Each “C” user having cash in the said farm must have the possibility of recovering the “rewards” (harvest) since his last “harvest” (resulting in 0.1% fees on the amount of the reward: see diagram)

* Each user “C” with cash in said farm must be able to withdraw cash from the farm

## Table of Contents
- [Yield Farming on Solana](#yield-farming-on-solana)
  - [Table of Contents](#table-of-contents)
  - [Quick Start](#quick-start)
  - [Learn about Solana](#learn-about-solana)
  - [Pointing to a public Solana cluster](#pointing-to-a-public-solana-cluster)

## Quick Start

The following dependencies are required to build and run this example, depending
on your OS, they may already be installed:

- Install node (v14 recommended)
- Install npm
- Install the latest Rust stable from https://rustup.rs/
- Install Solana later from
  https://docs.solana.com/cli/install-solana-cli-tools

If this is your first time using Rust, these [Installation
Notes](README-installation-notes.md) might be helpful.

### Configure CLI 

1. - Set CLI config url to localhost cluster

```bash
$ solana config set --url localhost
```

- Set CLI config url to devnet cluster

```bash
$ solana config set --url devnet
```

- Set CLI config url to mainnet-beta cluster

```bash
$ solana config set --url mainnet-beta
```

2. Create CLI Keypair

If this is your first time using the Solana CLI, you will need to generate a new keypair:

```bash
$ solana-keygen new
```

### Start local Solana cluster

This example connects to a local Solana cluster by default.

Start a local Solana cluster:
```bash
$ solana-test-validator
```
**WARNING: `solana-test-validator` is not currently available for native Windows.  Try using WSL, or switch to Linux or macOS**

Listen to transaction logs:
```bash
$ solana logs
```

### Install npm dependencies

```bash
$ npm install
```

### Build the on-chain program

There is both a Rust and C version of the on-chain program, whichever is built
last will be the one used when running the example.

```bash
$ npm run build:yf
```

### Deploy the on-chain program

```bash
$ npm run deploy:yf
```

## Learn about Solana

More information about how Solana works is available in the [Solana
documentation](https://docs.solana.com/) and all the source code is available on
[github](https://github.com/solana-labs/solana)

Further questions? Visit us on [Discord](https://discordapp.com/invite/pquxPsq)

## Learn about the client

The client in this example is written in TypeScript using:
- [Solana web3.js SDK](https://github.com/solana-labs/solana-web3.js)
- [Solana web3 API](https://solana-labs.github.io/solana-web3.js)

### Entrypoint

The [client's
entrypoint](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/main.ts#L13)
does five things.

### Establish a connection to the cluster

The client establishes a connection with the cluster by calling
[`establishConnection`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L92).

### Establish an account to pay for transactions

The client ensures there is an account available to pay for transactions,
and creates one if there is not, by calling
[`establishPayer`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L102).

## Learn about the on-chain program

The [on-chain helloworld program](/src/program-rust/Cargo.toml) is a Rust program
compiled to [Berkley Packet Format
(BPF)](https://en.wikipedia.org/wiki/Berkeley_Packet_Filter) and stored as an
[Executable and Linkable Format (ELF) shared
object](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format).

The program is written using:
- [Solana Rust SDK](https://github.com/solana-labs/solana/tree/master/sdk)

### Programming on Solana

To learn more about Solana programming model refer to the [Programming Model
Overview](https://docs.solana.com/developing/programming-model/overview).

To learn more about developing programs on Solana refer to the [On-Chain 
Programs Overview](https://docs.solana.com/developing/on-chain-programs/overview)

## Pointing to a public Solana cluster

Solana maintains three public clusters:
- `devnet` - Development cluster with airdrops enabled
- `testnet` - Tour De Sol test cluster without airdrops enabled
- `mainnet-beta` -  Main cluster

Use the Solana CLI to configure which cluster to connect to.

To point to `devnet`:
```bash
$ solana config set --url devnet
```

To point back to the local cluster:
```bash
$ solana config set --url localhost
```
