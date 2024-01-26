use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell}; 

#[derive(CandidType, Deserialize, Clone)]
struct Participant {
    address: String,
}

// Define the Event structure
#[derive(CandidType, Deserialize, Clone)]
struct Event {
    name: String,
    date: String,
    #[serde(default)] // This will default to an empty Vec if `participants` is not present
    participants: Vec<Participant>,
}

#[derive(CandidType, Deserialize)]
enum EventError {
    NoSuchEvent,
    JoinError,
    CancelJoinError,
    GetEventsError,
    AlreadyJoined,
    AlreadyExists
}

// Implement Storable for Event
impl Storable for Event {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 100;

// Implement BoundedStorable for Event
impl BoundedStorable for Event {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; // Adjust the size as needed
    const IS_FIXED_SIZE: bool = false;
}

// Initialize the events map with a new MemoryId
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static EVENTS_MAP: RefCell<StableBTreeMap<u64, Event, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), // Use a different MemoryId if needed
        )
    );
}

// create and store a new Event
#[ic_cdk::update]
fn create_event(name: String, date: String) -> Result<(), EventError> {
    EVENTS_MAP.with(|events_map_ref| {
        let mut events_map = events_map_ref.borrow_mut();

        // Check if an event with the same name and date already exists
        for (_, event) in events_map.iter() {
            if event.name == name && event.date == date {
                return Err(EventError::AlreadyExists);
            }
        }

        // If no existing event is found, create a new one
        let new_event = Event {
            name,
            date,
            participants: Vec::new(),
        };

        let new_event_id = events_map.len();
        events_map.insert(new_event_id, new_event);

        Ok(())
    })
}


// Get participants of an event
#[ic_cdk::update]
fn join_event(event_id: u64, participant_address: String) -> Result<(), EventError> {
    EVENTS_MAP.with(|events_map_ref| {
        let mut events_map = events_map_ref.borrow_mut();
        // Retrieve the event, clone it, and then modify it
        if let Some(mut event) = events_map.get(&event_id) {
            if event.participants.iter().any(|p| p.address == participant_address) {
                return Err(EventError::AlreadyJoined);
            }

            let new_participant = Participant {address: participant_address};
            event.participants.push(new_participant);
            // Insert the modified event back into the map
            events_map.insert(event_id, event);
            Ok(())
        } else {
            Err(EventError::NoSuchEvent)
        }
    })
}

// Function for a user to cancel joining an event
#[ic_cdk::update]
fn cancel_join_event(event_id: u64, participant_address: String) -> Result<(), EventError> {
    EVENTS_MAP.with(|events_map_ref| {
        let mut events_map = events_map_ref.borrow_mut();
        // Retrieve the event, clone it, and then modify it
        if let Some(mut event) = events_map.get(&event_id) {
            if let Some(index) = event
                .participants
                .iter()
                .position(|p| p.address == participant_address)
            {
                event.participants.remove(index);
                // Insert the modified event back into the map
                events_map.insert(event_id, event);
                Ok(())
            } else {
                Err(EventError::CancelJoinError)
            }
        } else {
            Err(EventError::NoSuchEvent)
        }
    })
}

// Query events in state
#[ic_cdk::query]
fn get_stored_events() -> Vec<Event> {
    EVENTS_MAP.with(|events_map| {
        events_map
            .borrow()
            .iter()
            .map(|(_, event)| event.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_event_by_id(event_id: u64) -> Option<Event> {
    EVENTS_MAP.with(|events_map| {
        let events = events_map.borrow();
        events.get(&event_id)
    })
}

// Query participants of given event
#[ic_cdk::query]
fn get_participants_of_event(event_id: u64) -> Option<Vec<String>> {
    EVENTS_MAP.with(|events_map| {
        let events = events_map.borrow();
        events.get(&event_id).map(|event| {
            event.participants.iter().map(|participant| participant.address.clone()).collect()
        })
    })
}


// Update method to make an HTTPS outcall and fetch events
#[ic_cdk::update]
async fn get_events_from_api() -> String {
    // Setup the URL for the HTTP GET request
    let url = "https://654c93da77200d6ba8590738.mockapi.io/events".to_string();

    // Prepare headers for the system http_request call
    let request_headers = vec![];

    // Setup the HTTP request arguments
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // Make the HTTPS request and wait for the response
    match http_request(request).await {
        Ok((response,)) => {
            if response.status == 200 {
                // Parse the JSON response into a Vec<Event>
                let events: Vec<Event> =
                    serde_json::from_slice(&response.body).expect("Failed to parse JSON response.");

                EVENTS_MAP.with(|events_map_ref| {
                    let mut events_map = events_map_ref.borrow_mut();
                    // Create a new map and fill it with the new events
                    let mut new_map = StableBTreeMap::init(
                        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
                    );
                    for (i, event) in events.into_iter().enumerate() {
                        new_map.insert(i as u64, event);
                    }
                    // Replace the old map with the new one
                    *events_map = new_map;
                });
                // Return a success message
                "Events fetched and stored successfully.".to_string()
            } else {
                format!("HTTP request failed with status code: {}", response.status)
            }
        }
        Err((code, message)) => {
            format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            )
        }
    }
}
