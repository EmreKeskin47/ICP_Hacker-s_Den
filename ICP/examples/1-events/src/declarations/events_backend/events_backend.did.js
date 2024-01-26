export const idlFactory = ({ IDL }) => {
  const EventError = IDL.Variant({
    'GetEventsError' : IDL.Null,
    'AlreadyJoined' : IDL.Null,
    'AlreadyExists' : IDL.Null,
    'JoinError' : IDL.Null,
    'NoSuchEvent' : IDL.Null,
    'CancelJoinError' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : EventError });
  const Participant = IDL.Record({ 'address' : IDL.Text });
  const Event = IDL.Record({
    'participants' : IDL.Vec(Participant),
    'date' : IDL.Text,
    'name' : IDL.Text,
  });
  return IDL.Service({
    'cancel_join_event' : IDL.Func([IDL.Nat64, IDL.Text], [Result], []),
    'create_event' : IDL.Func([IDL.Text, IDL.Text], [Result], []),
    'get_event_by_id' : IDL.Func([IDL.Nat64], [IDL.Opt(Event)], ['query']),
    'get_events_from_api' : IDL.Func([], [IDL.Text], []),
    'get_participants_of_event' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(IDL.Vec(IDL.Text))],
        ['query'],
      ),
    'get_stored_events' : IDL.Func([], [IDL.Vec(Event)], ['query']),
    'join_event' : IDL.Func([IDL.Nat64, IDL.Text], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
