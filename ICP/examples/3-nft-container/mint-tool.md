# DIP721 NFT container

This example demonstrates implementing an NFT canister. NFTs (non-fungible tokens) are unique tokens with arbitrary
metadata, usually an image of some kind, to form the digital equivalent of trading cards. There are a few different
NFT standards for the Internet Computer (e.g [EXT](https://github.com/Toniq-Labs/extendable-token), [IC-NFT](https://github.com/rocklabs-io/ic-nft)), but for the purposes of this tutorial we use [DIP-721](https://github.com/Psychedelic/DIP721). You can see a quick introduction on [YouTube](https://youtu.be/1po3udDADp4).

The canister is a basic implementation of the standard, with support for the minting, burning, and notification interface extensions.

## Overview

The NFT canister is not very complicated since the [DIP-721](https://github.com/Psychedelic/DIP721) standard specifies most [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) operations,
but we can still use it to explain three important concepts concerning dapp development for the Internet Computer:

A running instance of the Rust canister for demonstration purposes is available as [t5l7c-7yaaa-aaaab-qaehq-cai](https://t5l7c-7yaaa-aaaab-qaehq-cai.icp0.io).
The interface is meant to be programmatic, but the Rust version additionally contains HTTP functionality so you can view a metadata file at `<canister URL>/<NFT ID>/<file ID>`.
It contains six NFTs, so you can look at items from `<canister URL>/0/0` to `<canister URL>/5/0`.

### Prerequisites

-   [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
-   [x] Download and install [git.](https://git-scm.com/downloads)
-   [x] `wasm32-unknown-unknown` targets; these can be installed with `rustup target add wasm32-unknown-unknown`.

### Step 1: Clone the Github repo for the project's files and navigate into the directory:

```sh
git clone https://github.com/dfinity/examples
cd examples/rust/dip721-nft-container
```

### Step 2: Start the local replica before installing the canister:

```sh
dfx start --background --clean
```

### Step 3: Install the canister.

Deploy the canister with the command:

```sh
dfx deploy --no-wallet --argument \
"(record {
    name = \"Numbers One Through Fifty\";
    symbol = \"NOTF\";
    logo = opt record {
        data = \"$(base64 -i ./logo.png)\";
        logo_type = \"image/png\";
    };
    custodians = opt vec { principal \"$(dfx identity get-principal)\" };
})" dip721_nft_container
```

The canister expects a record parameter with the following fields:

-   `custodians`: A list of users allowed to manage the canister. If unset, it will default to the caller. If you're using `dfx`, and haven't specified `--no-wallet`, that's your wallet principal, not your own, so be careful!
-   `name`: The name of your NFT collection. Required.
-   `symbol`: A short slug identifying your NFT collection. Required.
-   `logo`: The logo of your NFT collection, represented as a record with fields `data` (the base-64 encoded logo) and `logo_type` (the MIME type of the logo file). If unset, it will default to the Internet Computer logo.

### Step 4: Interact with the canister.

Aside from the standard functions, it has five extra functions:

-   `set_name`, `set_symbol`, `set_logo`, and `set_custodian`: these functions update the collection information of the corresponding field from when it was initialized.
-   `is_custodian`: this function checks whether the specified user is a custodian.

The canister also supports a certified HTTP interface; going to `/<nft>/<id>` will return `nft`'s metadata file #`id`, with `/<nft>` returning the first non-preview file.

Remember that query functions are uncertified; the result of functions like `ownerOfDip721` can be modified arbitrarily by a single malicious node. If queried information is depended on, for example if someone might send ICP to the owner of a particular NFT to buy it from them, those calls should be performed as update calls instead. You can force an update call by passing the `--update` flag to `dfx` or using the `Agent::update` function in `agent-rs`.

### Step 5: Mint an NFT.

Due to size limitations on the length of a terminal command, an image- or video-based NFT would be impossible to send via `dfx`. To that end, there is an experimental [minting tool](https://github.com/dfinity/experimental-minting-tool) you can use to mint a single-file NFT.

To use this tool, install the minting tool with the command:

`cargo install --git https://github.com/dfinity/experimental-minting-tool --locked`

As an example, to mint the default logo, you would run the following command:

YOU MIGHT NEED TO EXECUTE
rustup default nightly && rustup update
ERROR: feature `edition2021` is required

channel = "stable"
rust-toolchain.toml

```sh
minting-tool local "$(dfx canister id dip721_nft_container)" --owner "$(dfx identity get-principal)" --file ./logo.png --sha2-auto
```

The output of this command should look like this:

```
Successfully minted token 0 to x4d3z-ufpaj-lpxs4-v7gmt-v56ze-aub3k-bvifl-y4lsq-soafd-d3i4k-fqe (transaction id 0)
```

Minting is restricted to anyone authorized with the `custodians` parameter or the `set_custodians` function. Since the contents of `--file` are stored on-chain, it's important to prevent arbitrary users from minting tokens, or they will be able to store arbitrarily-sized data in the contract and exhaust the canister's cycles. Be careful not to upload too much data to the canister yourself, or the contract will no longer be able to be upgraded afterwards.

YER BUL

Command-line length limitations would prevent you from minting an NFT with a large file, like an image or video, via `dfx`. To that end,
there is a [command-line minting tool](https://github.com/dfinity/experimental-minting-tool) provided for minting simple NFTs.

### Stopping any Running Network

```bash
dfx stop
```

Stops any currently running local DFINITY network.

### Setting Error Handling

```bash
set -e
trap 'dfx stop' EXIT
```

Configures the script to exit on any errors and ensures the DFINITY network is stopped when the script exits.
