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
  
#### Election Correctness:
  - ##### Safety: allow at most one winner per term
    - Each server gives only one vote per term (persisst on disk)
    - Majority required to win election
  - ##### Liveness: some candidate must eventually win
    - Choose election timeouts randomly in [T, 2T] e.g 150-300ms
    - One server usually times out and wins election before others time out
    - Works well if T >> broadcast time  
  Randomized approach simpler than ranking.
