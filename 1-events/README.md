# Events

The code defines a system for managing events, where each event can have multiple participants. It uses the Internet Computer's capabilities to store data in a stable memory and make HTTP requests.

### Key Structures

1. **Participant**

    - `address`: A `String` representing the address of a participant.

2. **Event**

    - `name`: A `String` representing the name of the event.
    - `date`: A `String` representing the date of the event.
    - `participants`: A `Vec<Participant>` storing the list of participants.

3. **EventError**
    - An enumeration defining possible errors such as `NoSuchEvent`, `JoinError`, `CancelJoinError`, `GetEventsError`, `AlreadyJoined`, and `AlreadyExists`.

### Storable Implementations

-   Both `Event` and `Participant` structures implement the `Storable` trait, allowing them to be serialized and deserialized for storage.

### Memory Management

-   The code uses `VirtualMemory` and `StableBTreeMap` to manage events in a stable memory structure.

### Functions

1. **create_event(name: String, date: String)**

    - Creates a new event with the given name and date.
    - Checks for existing events with the same name and date to avoid duplicates.

2. **join_event(event_id: u64, participant_address: String)**

    - Allows a participant to join an event by their address.
    - Checks if the participant has already joined to prevent duplicates.

3. **cancel_join_event(event_id: u64, participant_address: String)**

    - Allows a participant to cancel their participation in an event.

4. **get_stored_events()**

    - Returns a list of all stored events.

5. **get_event_by_id(event_id: u64)**

    - Retrieves a specific event by its ID.

6. **get_participants_of_event(event_id: u64)**

    - Returns a list of participants for a given event.

7. **get_events_from_api()**
    - Makes an HTTP request to an external API to fetch events.
    - Updates the stored events with the fetched data.

### HTTP Request Handling

-   The `get_events_from_api` function demonstrates how to make an HTTP GET request, handle the response, and update the stored events.

### Error Handling

-   The code includes comprehensive error handling, represented by the `EventError` enum, to manage different failure scenarios.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
