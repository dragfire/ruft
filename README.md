# ruft
Raft consensus protocol implementation in Rust

Node State:
  Follower (expects Heartbeat)
  Candidate (RequestVote RPC)
  Leader  (AppendEntries RPC: Replicate its logs, Heartbeats to maintain leadership)
 
