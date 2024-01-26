mod heartbeat;
mod types;

use crate::types::*;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::export::Principal;

thread_local! {
    static SERVICE: RefCell<BasicDaoService> = RefCell::default();
}

#[derive(Default)]
pub struct BasicDaoService {
    pub accounts: HashMap<Principal, Tokens>,
    pub proposals: HashMap<u64, Proposal>,
    pub next_proposal_id: u64,
    pub system_params: SystemParams,
}

impl From<BasicDaoStableStorage> for BasicDaoService {
    fn from(stable: BasicDaoStableStorage) -> BasicDaoService {
        let accounts = stable.accounts.clone().into_iter().map(|a| (a.owner, a.tokens)).collect();
        let proposals = stable.proposals.clone().into_iter().map(|p| (p.id, p)).collect();

        BasicDaoService {
            accounts,
            proposals,
            next_proposal_id: 1,
            system_params: stable.system_params,
        }
    }
}

//INITIALIZE
#[ic_cdk::init]
fn init(init_state: BasicDaoStableStorage) {
    ic_cdk::setup();

    // Convert BasicDaoStableStorage to BasicDaoService
    let init_service = BasicDaoService::from(init_state);

    // Store the initialized service in the thread-local SERVICE
    SERVICE.with(|service| *service.borrow_mut() = init_service);
}

//QUERIES
#[ic_cdk::query]
fn get_system_params() -> SystemParams {
    SERVICE.with(|service| service.borrow().system_params.clone())
}

#[ic_cdk::query]
fn account_balance() -> Tokens {
    SERVICE.with(|service| {
        let service = service.borrow();
        let caller = ic_cdk::api::caller();
        service.accounts.get(&caller).cloned().unwrap_or_default()
    })
}

#[ic_cdk::query]
fn list_accounts() -> Vec<Account> {
    SERVICE.with(|service| {
        service.borrow().accounts.iter().map(|(owner, tokens)| Account { owner: *owner, tokens: *tokens }).collect()
    })
}


#[ic_cdk::query]
fn get_proposal(proposal_id: u64) -> Option<Proposal> {
    SERVICE.with(|service| {
        service.borrow().proposals.get(&proposal_id).cloned()
    })
}

#[ic_cdk::query]
fn list_proposals() -> Vec<Proposal> {
    SERVICE.with(|service| {
        service.borrow().proposals.values().cloned().collect()
    })
}

#[ic_cdk::update]
fn transfer(args: TransferArgs) -> Result<(), String> {
    SERVICE.with(|service| {
        //due to service being mutable reference
        //In Rust, you cannot have a mutable borrow (service.borrow_mut()) and then try to access a field of the borrowed value 
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

#[ic_cdk::update]
fn submit_proposal(payload: ProposalPayload) -> Result<u64, String> {
    SERVICE.with(|service| {
        //due to service being mutable reference
        //In Rust, you cannot have a mutable borrow (service.borrow_mut()) and then try to access a field of the borrowed value 
        let proposal_submission_deposit = service.borrow().system_params.proposal_submission_deposit;
        let mut service = service.borrow_mut();
        let caller = ic_cdk::api::caller();

        if let Some(account) = service.accounts.get_mut(&caller) {
            if *account < proposal_submission_deposit {
                return Err("Insufficient funds to submit proposal".to_string());
            }
            *account -= proposal_submission_deposit;
        } else {
            return Err("Caller does not have an account".to_string());
        }

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
    })
}

#[ic_cdk::update]
fn vote(args: VoteArgs) -> Result<ProposalState, String> {
    let caller = ic_cdk::api::caller();

    // Get voting_power outside of the mutable borrow block
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

#[ic_cdk::update]
fn update_proposal_state(proposal_id: u64, new_state: ProposalState) {
    SERVICE.with(|service| {
        let mut service = service.borrow_mut();

        if let Some(proposal) = service.proposals.get_mut(&proposal_id) {
            proposal.state = new_state;
        }
    })
}


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
        if let Some(proposal_submission_deposit) = payload.proposal_submission_deposit {
            service.system_params.proposal_submission_deposit = proposal_submission_deposit;
        }
    })
}

