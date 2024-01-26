## Step 1: Setting Up the Environment

-   **Dependencies**:

    ```rust
    use ic_cdk::export::Principal;
    use std::cell::RefCell;
    use std::collections::HashMap;
    ```

    -   `ic_cdk`: Internet Computer's development kit for building canisters (smart contracts).
    -   `std`: Standard library in Rust, providing essential types and traits.

## Step 2: Defining Data Structures

-   **Account Struct**:

    ```rust
    #[derive(Clone, Debug, CandidType, Deserialize)]
    pub struct Account {
        pub owner: Principal,
        pub tokens: Tokens,
    }
    ```

    -   Represents an account with an owner and a token balance.

-   **Proposal Struct**:

    ```rust
    #[derive(Clone, Debug, CandidType, Deserialize)]
    pub struct Proposal {
        pub id: u64,
        pub timestamp: u64,
        pub proposer: Principal,
        pub payload: ProposalPayload,
        pub state: ProposalState,
        pub votes_yes: Tokens,
        pub votes_no: Tokens,
        pub voters: Vec<Principal>,
    }
    ```

    -   Represents a proposal within the DAO.

## Step 3: Implementing the BasicDaoService

-   **BasicDaoService Struct**:

    ```rust
    #[derive(Default)]
    pub struct BasicDaoService {
        pub accounts: HashMap<Principal, Tokens>,
        pub proposals: HashMap<u64, Proposal>,
        pub next_proposal_id: u64,
        pub system_params: SystemParams,
    }
    ```

    -   Manages accounts and proposals.

## Step 4: Initialization

-   **Initialization**:

    ```rust
    #[ic_cdk::init]
    fn init(init_state: BasicDaoStableStorage) {
        ic_cdk::setup();
        let init_service = BasicDaoService::from(init_state);
        SERVICE.with(|service| *service.borrow_mut() = init_service);
    }
    ```

    -   Initializes the DAO with a given state from arguements.

## Step 5: Implementing Core Functionalities

-   **Transfer Tokens**:

    ```rust
    #[ic_cdk::update]
    fn transfer(args: TransferArgs) -> Result<(), String> {
        SERVICE.with(|service| {
            let transfer_fee = service.borrow().system_params.transfer_fee;
            let mut service = service.borrow_mut();
            let caller = ic_cdk::api::caller();

            if let Some(account) = service.accounts.get_mut(&caller) {
                if *account < args.amount {
                    return Err(format!(
                        "Caller's account has insufficient funds to transfer {:?}",
                        args.amount
                    ));
                } else {
                    *account -= args.amount + transfer_fee;
                    let to_account = service.accounts.entry(args.to).or_default();
                    *to_account += args.amount;
                }
            } else {
                return Err("Caller needs an account to transfer funds".to_string());
            }

            Ok(())
        })
    }
    ```

    -   This function allows users to transfer tokens to another account. It checks if the caller has sufficient funds, deducts the transfer amount and fee, and credits the recipient's account.

-   **Submit Proposal**:

    ```rust
    #[ic_cdk::update]
    fn submit_proposal(payload: ProposalPayload) -> Result<u64, String> {
        SERVICE.with(|service| {
            let proposal_submission_deposit = service.borrow().system_params.proposal_submission_deposit;
            let mut service = service.borrow_mut();
            let caller = ic_cdk::api::caller();

            if let Some(account) = service.accounts.get_mut(&caller) {
                if *account < proposal_submission_deposit {
                    return Err("Insufficient funds to submit proposal".to_string());
                }
                *account -= proposal_submission_deposit;

                let proposal_id = service.next_proposal_id;
                service.next_proposal_id += 1;

                let new_proposal = Proposal {
                    id: proposal_id,
                    timestamp: ic_cdk::api::time(),
                    proposer: caller,
                    payload,
                    state: ProposalState::Open,
                    votes_yes: Default::default(),
                    votes_no: Default::default(),
                    voters: Vec::new(),
                };

                service.proposals.insert(proposal_id, new_proposal);
                Ok(proposal_id)
            } else {
                return Err("Caller does not have an account".to_string());
            }
        })
    }
    ```

    -   This function allows users to submit new proposals. It checks if the user has enough tokens for the submission deposit, creates a new proposal, and adds it to the proposals map.

-   **Vote on Proposal**:

    ```rust
    #[ic_cdk::update]
    fn vote(args: VoteArgs) -> Result<ProposalState, String> {
        let caller = ic_cdk::api::caller();

        let voting_power = SERVICE.with(|service| {
            service.borrow().accounts.get(&caller)
                .cloned()
                .ok_or_else(|| "Caller does not have an account".to_string())
        })?;

        SERVICE.with(|service| {
            let proposal_vote_threshold = service.borrow().system_params.proposal_vote_threshold;
            let mut service = service.borrow_mut();

            let proposal = service.proposals.get_mut(&args.proposal_id)
                .ok_or_else(|| "Proposal not found".to_string())?;

            if proposal.voters.contains(&caller) {
                return Err("Caller has already voted".to_string());
            }

            match args.vote {
                Vote::Yes => proposal.votes_yes += voting_power,
                Vote::No => proposal.votes_no += voting_power,
            }

            proposal.voters.push(caller);

            if proposal.votes_yes >= proposal_vote_threshold {
                proposal.state = ProposalState::Accepted;
            } else if proposal.votes_no >= proposal_vote_threshold {
                proposal.state = ProposalState::Rejected;
            }

            Ok(proposal.state.clone())
        })
    }
    ```

    -   This function enables users to vote on proposals. It checks if the user has already voted, updates the vote count, and changes the proposal state based on the voting outcome.

## Step 6: Heartbeat Functionality

-   **Heartbeat Implementation**:

    ```rust
    #[heartbeat]
    async fn heartbeat() {
        let accepted_proposals: Vec<u64> = SERVICE.with(|service| {
            service.borrow_mut()
                .proposals
                .iter_mut()
                .filter(|(_, proposal)| proposal.state == ProposalState::Accepted)
                .map(|(id, proposal)| {
                    proposal.state = ProposalState::Executing;
                    *id
                })
                .collect()
        });

        for proposal_id in accepted_proposals {
            let state = match execute_proposal(proposal_id).await {
                Ok(()) => ProposalState::Succeeded,
                Err(msg) => ProposalState::Failed(msg)
            };

            update_proposal_state(proposal_id, state);
        }
    }
    ```

    -   This function is called at regular intervals (heartbeat). It checks for accepted proposals, executes them, and updates their state based on the execution result.

## Step 7: System Parameters and Updates

```rust
#[derive(Clone, Default, Debug, CandidType, Deserialize)]
pub struct SystemParams {
    pub transfer_fee: Tokens,
    pub proposal_vote_threshold: Tokens,
    pub proposal_submission_deposit: Tokens,
}
```

### Fields

1. transfer_fee

    - This field represents the fee incurred whenever tokens are transferred between accounts within the DAO. The fee is deducted from the sender's account in addition to the amount being transferred.
    - The primary purpose of this fee is to prevent spam transactions and to potentially generate revenue for the DAO, which can be used for various purposes as decided by the DAO governance.
    - It's used in the `transfer` function to calculate the total amount deducted from the sender's account.

2. proposal_vote_threshold

    - This field specifies the minimum amount of tokens that must be accumulated in favor or against a proposal for it to be accepted or rejected, respectively.
    - The threshold ensures that a proposal is only accepted or rejected if there is a significant amount of consensus among the token holders, thus fostering a more democratic decision-making process.
    - It's utilized in the `vote` function to determine whether the accumulated votes on a proposal are sufficient to change its state to either accepted or rejected.

3. proposal_submission_deposit

    - This field indicates the number of tokens that are temporarily deducted from a user's account when they submit a proposal. If the proposal is accepted, this deposit is returned to the user; if it is rejected or fails to meet the vote threshold, the deposit is forfeited.
    - The deposit acts as a deterrent against the submission of spam proposals, ensuring that users submit proposals thoughtfully and responsibly.
    - This deposit is checked and deducted in the `submit_proposal` function. It's a critical part of the proposal submission process, ensuring the seriousness and commitment of the proposer.

-   **Update System Parameters**:

    ```rust
    #[ic_cdk::update]
    fn update_system_params(payload: UpdateSystemParamsPayload) {
        SERVICE.with(|service| {
            let mut service = service.borrow_mut();
            let caller = ic_cdk::api::caller();

            if caller != ic_cdk::api::id() {
                // Only the canister itself can update system parameters
                return;
            }

            if let Some(transfer_fee) = payload.transfer_fee {
                service.system_params.transfer_fee = transfer_fee;
            }

            if let Some(proposal_vote_threshold) = payload.proposal_vote_threshold {
                service.system_params.proposal_vote_threshold = proposal_vote_threshold;
            }
        });
    }
    ```

    -   This function allows the canister itself to update system parameters like `transfer_fee` and `proposal_vote_threshold`.

## Step 8: Implementing the DID file

Here is the completed DID version of our canister

```
type BasicDaoStableStorage = record {
    accounts: vec Account;
    proposals: vec Proposal;
    system_params: SystemParams;
};

type Tokens = record {
    amount_e8s: nat64;
};

type ProposalState = variant {
    Open;
    Accepted;
    Rejected;
    Executing;
    Succeeded;
    Failed: text;
};

type Proposal = record {
    id: nat64;
    timestamp: nat64;
    proposer: principal;
    payload: ProposalPayload;
    state: ProposalState;
    votes_yes: Tokens;
    votes_no: Tokens;
    voters: vec principal;
};

type ProposalPayload = record {
    canister_id: principal;
    method: text;
    message: blob;
};

type SubmitProposalResult = variant {
    Ok: nat64;
    Err: text;
};

type Vote = variant {
    Yes;
    No;
};

type Account = record {
    owner: principal;
    tokens: Tokens;
};

type TransferArgs = record {
    to: principal;
    amount: Tokens;
};

type TransferResult = variant {
    Ok;
    Err: text;
};

type VoteArgs = record {
    proposal_id: nat64;
    vote: Vote;
};

type VoteResult = variant {
    Ok: ProposalState;
    Err: text;
};

type SystemParams = record {
    transfer_fee: Tokens;
    proposal_vote_threshold: Tokens;
    proposal_submission_deposit: Tokens;
};

type UpdateSystemParamsPayload = record {
    transfer_fee: opt Tokens;
    proposal_vote_threshold: opt Tokens;
    proposal_submission_deposit: opt Tokens;
};

service : (BasicDaoStableStorage) -> {
    get_system_params: () -> (SystemParams);
    transfer: (TransferArgs) -> (TransferResult);
    account_balance: () -> (Tokens) query;
    list_accounts: () -> (vec Account) query;
    submit_proposal: (ProposalPayload) -> (SubmitProposalResult);
    get_proposal: (nat64) -> (opt Proposal);
    list_proposals: () -> (vec Proposal);
    vote: (VoteArgs) -> (VoteResult);
    update_system_params: (UpdateSystemParamsPayload) -> ();
}

```

## Step 9: Testing and Deployment

-   **Deployment**:
    -   Deploy the contract to the Internet Computer network using `dfx deploy`.
-   **Testing**:
    -   See README.md for details
