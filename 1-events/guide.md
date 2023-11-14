# Guide on how to implement

#### Step 1: Importing Dependencies

-   **Import Statements**:
    ```rust
    use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpMethod};
    use candid::{CandidType, Decode, Deserialize, Encode};
    use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
    use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
    use std::{borrow::Cow, cell::RefCell};
    ```
    -   `ic_cdk`: Internet Computer's development kit for building canisters (smart contracts).
    -   `candid`: A serialization/deserialization library for DApps.
    -   `ic_stable_structures`: Libraries for stable memory management in Internet Computer.
    -   `std`: Standard library in Rust, providing essential types and traits.

#### Step 2: Defining Data Structures

-   **Participant Struct**:

    ```rust
    #[derive(CandidType, Deserialize, Clone)]
    struct Participant {
        address: String,
    }
    ```

    -   `#[derive(CandidType, Deserialize, Clone)]`: Automatically implements traits for serialization, deserialization, and cloning.
    -   `struct Participant`: Defines a struct to represent a participant with an `address` field.

-   **Event Struct**:
    ```rust
    #[derive(CandidType, Deserialize, Clone)]
    struct Event {
        name: String,
        date: String,
        #[serde(default)]
        participants: Vec<Participant>,
    }
    ```
    -   `struct Event`: Represents an event with `name`, `date`, and `participants`.
    -   `Vec<Participant>`: A dynamic array to store multiple participants.
    -   `#[serde(default)]`: If `participants` is missing in the data, it defaults to an empty vector.

#### Step 3: Implementing Traits for Event

The Storable trait is implemented to allow Event objects to be stored in and retrieved from the DApp's memory. This is essential for persisting event data across sessions and interactions.

StableBTreeMap relies on serialization and deserialization to store and retrieve data. When data is saved, it's converted into a byte format suitable for storage in stable memory. When data is read, it's converted back into its original Rust data structure.

-   **Storable Trait for Event**:

    ```rust
    impl Storable for Event {
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }

        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }
    }
    ```

    -   `impl Storable for Event`: Implements the `Storable` trait for `Event`.
    -   `to_bytes`: Converts `Event` to a byte array for storage.
    -   `from_bytes`: Reconstructs `Event` from a byte array.
    -   `Cow<[u8]>`: Ensures efficient memory usage during conversion.

-   **BoundedStorable Trait for Event**:

This trait implementation provides additional information about the storage requirements of Event objects, such as MAX_SIZE

```rust
impl BoundedStorable for Event {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}
```

-   `impl BoundedStorable for Event`: Implements the `BoundedStorable` trait.
-   `MAX_SIZE`: Maximum size in bytes that an `Event` can occupy.
-   `IS_FIXED_SIZE`: Indicates that `Event` does not have a fixed size.

#### Step 4: Memory Management and Event Map Initialization

By utilizing StableBTreeMap in conjunction with a MemoryManager, we create a system where event information is consistently maintained and accessible, regardless of updates or changes to the DApp's codebase.

-   **Memory Management**:

    ```rust
    thread_local! {
        static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
            RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

        static EVENTS_MAP: RefCell<StableBTreeMap<u64, Event, Memory>> = RefCell::new(
            StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            )
        );
    }
    ```

    -   `thread_local!`: Declares thread-local static variables.
    -   `RefCell`: Enables mutable access to the `MemoryManager` and `EVENTS_MAP`.
    -   `MemoryManager`: Manages the allocation and deallocation of stable memory.
    -   `StableBTreeMap`: A map structure to store events,for those requiring persistent storage across canister upgrades

#### Step 5: Implementing Event Management Functions

In this function, a closure is used with the EVENTS_MAP.with method to safely access and modify the event map. This closure checks for existing events and, if none are found, inserts a new one.

-   **Create Event Function**:

    ```rust
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
    ```

    -   `#[ic_cdk::update]`: Marks the function as an update call in the Internet Computer, modifying the state.
    -   `create_event`: Adds a new event to the `EVENTS_MAP`.
    -   The with method accepts a closure as an argument. This closure is executed with the value contained within the RefCell of the EVENTS_MAP. Essentially, it provides a safe way to access and modify the EVENTS_MAP.
    -   closure takes one argument, events_map_ref, which is a reference to the RefCell containing the StableBTreeMap.
    -   Inside the closure, the borrow_mut method is called on events_map_ref to get a mutable reference to the StableBTreeMap. This is where the closure captures the events_map_ref from its environment.
    -   The closure returns a Result<(), EventError>, which is also the return type of the create_event function.

-   **Join Event Function**:

    ```rust
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
    ```

    -   Adds a participant to an event.
    -   Handles potential errors like `AlreadyJoined` or `NoSuchEvent`.

-   **Cancel Join Event Function**:

    ```rust
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
    ```

    -   Removes a participant from an event.
    -   Manages errors like `CancelJoinError` or `NoSuchEvent`.

-   **Get Stored Events Function**:

    ```rust
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
    ```

    -   `#[ic_cdk::query]`: Marks the function as a query call, which does not modify the state.
    -   Retrieves all stored events.

-   **Get Event by ID Function**:
    ```rust
    #[ic_cdk::query]
    fn get_event_by_id(event_id: u64) -> Option<Event> {
        EVENTS_MAP.with(|events_map| {
            let events = events_map.borrow();
            events.get(&event_id)
        })
    }
    ```
    -   Fetches a specific event by its ID.

#### Step 6: Add HTTP Get for create events

This function is designed to fetch event data from an external API and update the canister's state with the retrieved information.

```rust
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
```

# Making HTTPS Requests from Canisters in the Internet Computer Protocol (ICP)

## Function Overview: `get_events_from_api`

The `get_events_from_api` function in Rust demonstrates how to make HTTPS requests from a canister in the Internet Computer Protocol (ICP). This function is designed to fetch event data from an external API and update the canister's state with the retrieved information.

### Code Explanation

```rust
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
        // Handling the response...
    }
}
```

-   **Note:** The function is marked `async`, indicating that the HTTP request is performed asynchronously. This is crucial in a distributed environment like ICP, where network calls can have variable latency.

-   `http_request(request).await`: This line sends an asynchronous HTTP GET request to the specified URL and waits for the response.
-   The response is handled within a `match` statement.

    -   On success, the response is parsed, and the events are stored in the canister's state.
    -   On failure, an appropriate error message is generated.

-   The response body is parsed from JSON into a vector of `Event` structs.

#### Step 7: DID adjustments

-   **DID File Explanation**:

    -   The `events_backend.did` file defines the interface for the DApp.
    -   Explains how each service function is accessible from the frontend.

    Here is the completed version of the did file

    ```
    type Participant = record {
    address: text;
    };

    type Event = record {
    name: text;
    date: text;
    participants: vec Participant;
    };

    type Result =
    variant {
    Ok;
    Err: EventError;
    };

    type EventError =
    variant {
    NoSuchEvent;
    JoinError;
    CancelJoinError;
    GetEventsError;
    AlreadyJoined;
    AlreadyExists;
    };

    service : {
    "get_events_from_api": () -> (text);
    "create_event": (text,text) -> (Result);
    "join_event": (nat64, text) -> (Result);
    "cancel_join_event": (nat64, text) -> (Result);
    "get_stored_events": () -> (vec Event) query;
    "get_participants_of_event": (nat64) -> (opt vec text) query;
    "get_event_by_id": (nat64) -> (opt Event) query;
    }
    ```

#### Step 8: Testing and Deployment

-   **Deployment**:
    -   execute dfx deploy
