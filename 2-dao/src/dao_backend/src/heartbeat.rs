use ic_cdk_macros::heartbeat;
use crate::{SERVICE, update_proposal_state};
use crate::types::ProposalState;

#[heartbeat]
async fn heartbeat() {
    execute_accepted_proposals().await;
}

/// Execute all accepted proposals
async fn execute_accepted_proposals() {
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

/// Execute the given proposal
async fn execute_proposal(proposal_id: u64) -> Result<(), String> {
    // Retrieve the proposal details from the SERVICE
    let proposal = SERVICE.with(|service| {
        service.borrow().proposals.get(&proposal_id).cloned()
    }).ok_or_else(|| "Proposal not found".to_string())?;

    ic_cdk::api::call::call_raw(
        proposal.payload.canister_id,
        &proposal.payload.method,
        &proposal.payload.message.clone(),
        0
    ).await
        .map_err(|(code, msg)| {
            format!(
                "Proposal execution failed: \
                canister: {}, method: {}, rejection code: {:?}, message: {}",
                proposal.payload.canister_id,
                &proposal.payload.method,
                code, msg
            )
        })
        .map(|_| ())
}
