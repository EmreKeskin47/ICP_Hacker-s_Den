# NFT Container

This document outlines the steps involved in the NFT application workflow using the `dfx` command-line tool. It demonstrates how to mint and transfer NFTs among users named ALICE and BOB.

## Setup and Deployment

### Step 1:Starting the Local Network

```bash
dfx start --background --clean
```

Starts a fresh local DFINITY network in the background.

## Identity Setup

### Creating Identities for ALICE and BOB

```bash
dfx identity new --disable-encryption Alice
dfx identity new --disable-encryption Bob
```

Creates two new identities, ALICE and BOB, for the demonstration.

-   **Note**: Since the last created identity is BOB, current identity is also BOB

### Step 2: Deploying the NFT Canister

```bash
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

## NFT Operations

### Minting an NFT for BOB

```bash
dfx canister call dip721_nft_container mintDip721 "(principal\"$(dfx identity get-principal)\",vec{record{
        purpose=variant{Rendered};
        data=blob\"hello\";
        key_val_data=vec{
            record{
                \"contentType\";
                variant{TextContent=\"text/plain\"};
            };
            record{
                \"locationType\";
                variant{Nat8Content=4:nat8}
            };
        }
    }},blob\"hello\")"
```

#### DEATAILS ABOUT THE METADATA

The metadata vector vec{...} contains a single record with the following fields:

-   **purpose** : variant{Rendered}: This specifies the purpose of the metadata. In this case, Rendered suggests that the metadata is intended for rendering or display purposes.
-   **data**: blob\"hello\": This is the actual data of the metadata, which in this case is a simple string "hello" encoded as a blob.
-   **key_val_data**: vec{...}: This is a vector of key-value pairs providing additional details about the metadata
    -   record{"contentType"; variant{TextContent="text/plain"}}: This record indicates the content type of the NFT. Here, it's specified as plain text ("text/plain").
    -   record{"locationType"; variant{Nat8Content=4:nat8}}: This specify type of location or categorization for the NFT, represented as an 8-bit natural number (nat8) with a value of 4

Mints a new NFT with metadata "hello" for the current user.

### Querying NFT Metadata

```bash
dfx canister call dip721_nft_container getMetadataDip721 '(0:nat64)'
```

Retrieves metadata for the newly created NFT.

### Checking NFT Ownership and Balances

```bash
dfx canister call dip721_nft_container ownerOfDip721 '(0:nat64)'
dfx canister call dip721_nft_container balanceOfDip721 "(principal\"$(dfx identity get-principal)\")"
dfx canister call dip721_nft_container balanceOfDip721 "(principal\"$(dfx --identity ALICE identity get-principal)\")"
```

Response:

```sh
(
  variant {
    Ok = principal "pnuea-ed6sg-rm3zf-kjs5d-oa4lm-rl3bp-in2gv-zlrpo-qloe3-kiolh-5ae"
  },
)
(1 : nat64)
(0 : nat64)
```

Checks the owner of NFT 0 and queries the number of NFTs owned by the current user and ALICE.

### Transferring an NFT from BOB to ALICE

```bash
dfx canister call dip721_nft_container transferFromDip721 "(principal\"$(dfx identity get-principal)\",principal\"$(dfx --identity ALICE identity get-principal)\",0:nat64)"
```
