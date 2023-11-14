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

### Functions

#### Initialization and Upgrade

-   **`init(args: InitArgs)`**: Initializes the canister with given arguments.
-   **`pre_upgrade()`**: Prepares and serializes the canister's state before an upgrade.
-   **`post_upgrade()`**: Restores the canister's state after an upgrade.

#### NFT Management

-   **`mint(to: Principal, metadata: MetadataDesc, blob_content: Vec<u8>)`**: Mints a new NFT.
-   **`burn(token_id: u64)`**: Burns (destroys) an NFT.

#### NFT Transfer

-   **`transfer_from(from: Principal, to: Principal, token_id: u64)`**: Transfers an NFT from one principal to another.
-   **`safe_transfer_from(from: Principal, to: Principal, token_id: u64)`**: Safely transfers an NFT, checking for zero addresses.
-   **`transfer_from_notify(from: Principal, to: Principal, token_id: u64, data: Vec<u8>)`**: Transfers an NFT and notifies the recipient.
-   **`safe_transfer_from_notify(from: Principal, to: Principal, token_id: u64, data: Vec<u8>)`**: Safely transfers an NFT with notification.

#### Approval and Access Control

-   **`approve(user: Principal, token_id: u64)`**: Approves a user to manage a specific NFT.
-   **`set_approval_for_all(operator: Principal, is_approved: bool)`**: Sets or unsets approval for an operator to manage all NFTs of the caller.
-   **`is_approved_for_all(operator: Principal)`**: Checks if an operator is approved for all NFTs of the caller.
-   **`set_custodian(user: Principal, custodian: bool)`**: Adds or removes a custodian.
-   **`is_custodian(principal: Principal)`**: Checks if a principal is a custodian.

#### Query Functions

-   **`balance_of(user: Principal)`**: Returns the number of NFTs owned by a user.
-   **`owner_of(token_id: u64)`**: Returns the owner of a specific NFT.
-   **`name()`**: Returns the name of the NFT collection.
-   **`symbol()`**: Returns the symbol of the NFT collection.
-   **`total_supply()`**: Returns the total number of NFTs minted.
-   **`supported_interfaces()`**: Lists the supported interfaces (DIP721 standards).
-   **`get_metadata(token_id: u64)`**: Retrieves metadata for a specific NFT.
-   **`get_metadata_for_user(user: Principal)`**: Retrieves metadata for all NFTs owned by a user.

#### Customization Functions

-   **`set_name(name: String)`**: Sets the name of the NFT collection.
-   **`set_symbol(sym: String)`**: Sets the symbol of the NFT collection.
-   **`set_logo(logo: Option<LogoResult>)`**: Sets the logo for the NFT collection.

### ICP-Example Repo

The sample code is available in the [samples repository](https://github.com/dfinity/examples) in [Rust](https://github.com/dfinity/examples/tree/master/rust/dip721-nft-container)

#### Demo

This Rust example comes with a demo script, `demo.sh`, which runs through an example workflow with minting and trading an NFT between a few users. This is primarily designed to be read rather than run so that you can use it to see how basic NFT operations are done. For a more in-depth explanation, read the [standard][DIP721].

### 1 - Open Terminal

First, open the Terminal application on your Mac. You can find it in the Applications folder under Utilities, or you can search for it using Spotlight.

### 2- Make the Script Executable

Before running the script, you need to make sure it is executable. You can do this by running the following command:

```sh
chmod +x demo.sh
```

### 3- Run the Script
Now, you can run the script by typing:

```sh
./demo.sh
```
