import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Event {
  'participants' : Array<Participant>,
  'date' : string,
  'name' : string,
}
export type EventError = { 'GetEventsError' : null } |
  { 'AlreadyJoined' : null } |
  { 'AlreadyExists' : null } |
  { 'JoinError' : null } |
  { 'NoSuchEvent' : null } |
  { 'CancelJoinError' : null };
export interface Participant { 'address' : string }
export type Result = { 'Ok' : null } |
  { 'Err' : EventError };
export interface _SERVICE {
  'cancel_join_event' : ActorMethod<[bigint, string], Result>,
  'create_event' : ActorMethod<[string, string], Result>,
  'get_event_by_id' : ActorMethod<[bigint], [] | [Event]>,
  'get_events_from_api' : ActorMethod<[], string>,
  'get_participants_of_event' : ActorMethod<[bigint], [] | [Array<string>]>,
  'get_stored_events' : ActorMethod<[], Array<Event>>,
  'join_event' : ActorMethod<[bigint, string], Result>,
}
