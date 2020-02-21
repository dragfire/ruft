# ruft
Raft consensus protocol implementation in Rust

#### Node State:
  - Follower (expects Heartbeat)
  - Candidate (RequestVote RPC)
  - Leader  (AppendEntries RPC: Replicate its logs, Heartbeats to maintain leadership)
 
#### Terms:
  - At most 1 leader per term
  - Some terms have no leader (failed election)
  - Each server maintains current term value(no global view)
    - Exchanged in every RPC
    - Peer has later term? Update term, revert to follower
    - Incoming RPC has obsolete term? Reply with error
  Terms identify obsolete information

#### Leader Election:
  - Become Candidate (relies on Election Timeout)
  - currentTerm++, vote for self
  - send RequestVote RPCs to other servers (if timeout, go to above step)
    - votes from majority: Become leader, send heartbeats
    - RPC from leader: become follower
  
