# Basic decentralized autonomous organization (DAO)

This sample project demonstrates a basic [decentralized autonomous organization](https://en.wikipedia.org/wiki/Decentralized_autonomous_organization) (DAO) that can be deployed to the [Internet Computer](https://github.com/dfinity/ic). The basic DAO sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/dao_backend) and [Rust](https://github.com/dfinity/examples/tree/master/rust/dao_backend). You can see a quick introduction on [YouTube](https://youtu.be/3IcYlieA-EE).

## Overview

The `dao_backend` can be initialized with a set of accounts: mappings from principal IDs to an amount of tokens.Account owners can query their account balance by calling `account_balance` and transfer tokens to other accounts by calling `transfer`. Anyone can call `list_accounts` to view all accounts.

Account owners can submit proposals by calling `submit_proposal`. A proposal specifies a canister, method and arguments for this method. Account owners can cast votes (either `Yes` or `No`) on a proposal by calling `vote`. The amount of votes cast is equal to amount of tokens the account owner has. If enough `Yes` votes are cast, `dao_backend` will execute the proposal by calling the proposal’s given method with the given args against the given canister. If enough `No` votes are cast, the proposal is not executed, and is instead marked as `Rejected`.

Certain system parameters, like the number of `Yes` votes needed to pass a proposal, can be queried by calling `get_system_params`. These system params can be modified via the proposal process, i.e. a proposal can be made to call `update_system_params` with updated values. The below demo does exactly that.

View the [canister service definition](https://github.com/dfinity/examples/blob/master/rust/dao_backend/src/dao_backend/src/dao_backend.did) for more details.
Begin by opening a terminal window.

### Step 1: Install prerequisites

This example requires an installation of:

-   [x] The Rust toolchain (e.g. cargo).
-   [x] [didc.](https://github.com/dfinity/candid/tree/master/tools/didc)
-   [x] Install the [IC SDK](../developer-docs/setup/install/index.mdx).

### Step 2: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```
cd dao_backend
dfx start --background
```

### Step 3: Create test identities with the commands:

```
dfx identity new --disable-encryption Alice; dfx identity use Alice; export ALICE=$(dfx identity get-principal);
dfx identity new --disable-encryption Bob; dfx identity use Bob; export BOB=$(dfx identity get-principal);
```

### Step 4: Deploy dao_backend with the initial test accounts:

```
dfx deploy --argument "(record {
 accounts = vec { record { owner = principal \"$(dfx --identity ALICE identity get-principal)\"; tokens = record { amount_e8s = 100_000_000:nat64 }; };
                  record { owner = principal \"$(dfx --identity BOB identity get-principal)\"; tokens = record { amount_e8s = 100_000_000:nat64 };}; };
 proposals = vec {};
 system_params = record {
     transfer_fee = record { amount_e8s = 10_000:nat64 };
     proposal_vote_threshold = record { amount_e8s = 10_000_000:nat64 };
     proposal_submission_deposit = record { amount_e8s = 10_000:nat64 };
 };
})" dao_backend
```

deployment yields an output similar to the following:

```
Backend canister via Candid interface:
    dao_backend: http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai
```

-   Note: We will need the canister id from here later on

### Step 5: List accounts and confirm you see the two test accounts:

```
dfx canister call dao_backend list_accounts
```

Output:

```
(
  vec {
    record {
      owner = principal "5l3ql-7jlet-6yy5p-fk2ud-e7qul-6vqqx-cnqvu-zq75f-r76jx-tf6gb-2ae";
      tokens = record { amount_e8s = 100_000_000 : nat };
    };
    record {
      owner = principal "gbr7o-qdqaz-fm5ds-xrg4l-k7bwl-6m3vk-tvjas-ner6w-wt2hq-hiav7-3ae";
      tokens = record { amount_e8s = 100_000_000 : nat };
    };
  },
)
```

### Step 6: Call `account_balance` as Bob:

```
dfx canister call dao_backend account_balance
```

You should see the output:

```
(record { amount_e8s = 100_000_000 : nat64 })
```

### Step 7: Transfer tokens to Alice:

```
dfx canister call dao_backend transfer "(record { to = principal \"$(dfx --identity ALICE identity get-principal)\"; amount = record { amount_e8s = 90_000_000: nat64;};})";
```

Output:

`(variant { Ok })`

### Step 8: List accounts and see that the transfer was made:

Switch to BOB

```
dfx identity use BOB
```

```
dfx canister call dao_backend list_accounts '()'
```

Output:

```
 (
   vec {
     record {
       owner = principal "$ALICE";
       tokens = record { amount_e8s = 190_000_000 : nat64 };
     };
     record {
       owner = principal "$BOB";
       tokens = record { amount_e8s = 9_990_000 : nat64 };
     };
   },
 )
```

:::info
Note that the transfer fee was deducted from Bob's account.
:::

### Step 9: Let's make a proposal to change the transfer fee. We can call `get_system_params` to learn the current transfer fee:

```
dfx canister call dao_backend get_system_params '()';
```

Output:

```
(
  record {
    transfer_fee = record { amount_e8s = 10_000 : nat64 };
    proposal_vote_threshold = record { amount_e8s = 10_000_000 : nat64 };
    proposal_submission_deposit = record { amount_e8s = 10_000 : nat64 };
  },
)
```

We can change `transfer_fee` by calling `dao_backend`'s `update_system_params` method. This method takes a `UpdateSystemParamsPayload` as an arg, which we need to encode into a blob to use in `ProposalPayload`. Use didc to encode a `UpdateSystemParamsPayload`:

`didc encode '(record { transfer_fee = opt record { amount_e8s = 20_000:nat64; }; })' -f blob`

which will yield:

blob "DIDL\03l\01\f2\c7\94\ae\03\01n\02l\01\b9\ef\93\80\08x\01\00\01 N\00\00\00\00\00\00"

### Step 10: We can then submit the proposal:

```
dfx canister call dao_backend submit_proposal '(record { canister_id = principal "bkyz2-fmaaa-aaaaa-qaaaq-cai";
method = "update_system_params":text;
message = blob "DIDL\03l\01\f2\c7\94\ae\03\01n\02l\01\b9\ef\93\80\08x\01\00\01 N\00\00\00\00\00\00"; })'
```

Note the output proposal ID:

```
(variant { Ok = 0 : nat64 })
```

### Step 11: Confirm the proposal was created:

```
dfx canister call dao_backend get_proposal '(1:nat64)'
```

You should see `state = variant { Open };` in the output.

### Step 12: Vote on the proposal:

```
dfx canister call dao_backend vote '(record { proposal_id = 1:nat64; vote = variant { Yes };})'
```

You should see the following output:

```
(variant { Ok = variant { Open } })
```

Because we voted as Bob, and Bob does not have enough voting power to pass proposals, the proposal remains Open. To get the proposal accepted, we can vote with Alice:

```
dfx identity use Alice; dfx canister call dao_backend vote '(record { proposal_id = 1:nat64; vote = variant { Yes };})';
```

You should see the following output:

```
(variant { Ok = variant { Accepted } })
```

Query the proposal again:

```
dfx canister call dao_backend get_proposal '(1:nat64)'
```

And see that the state is `Succeeded`:

```
state = variant { Succeeded };
```

Query the system params again and see that transfer_fee has been updated:

```
dfx canister call dao_backend get_system_params '()'
```

Output:

```
(
  record {
    transfer_fee = record { amount_e8s = 20_000 : nat64 };
    proposal_vote_threshold = record { amount_e8s = 10_000_000 : nat64 };
    proposal_submission_deposit = record { amount_e8s = 10_000 : nat64 };
  },
)
```

### Resources

-   [ic-cdk](https://docs.rs/ic-cdk/latest/ic_cdk/)
-   [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
-   [JavaScript API reference](https://erxue-5aaaa-aaaab-qaagq-cai.ic0.app/)
