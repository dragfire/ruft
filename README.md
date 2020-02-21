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
  
#### Normal Operation:
  - Client sends command to leader
  - Leader appends command to its log
  - Leader sends AppendEntries RPCs to all followers
  - Once new entry committed:
    - Leader executes command in its state machine, returns result to client
    - Leader notifies followers of committed entres in subsequent AppendEntries RPCs
    - Followers execute committed commands in their state machines 
  - Crashed/slow followers?
    - Leader retries AppendEntries RPCs until they succeed
  - Optimal performance in common case
    - One successful RPC to any majority of servers
 
#### Log Structure:
 Each node has its own log.
  - Must survive crashes (store on disk)
  - Entry committed if safe to execute in state machines
    - Replicated on majority of servers by leader of its term
    
#### Log Inconsistencies:
  Crashes lead to inconsistencies.
  Raft minimizes special code for repairing inconsistencies:
    - Leader assumes its log is correct
    - Normal operation will repair all inconsistencies

#### Log Matching Property:
  - If log entries on different servers have same index and term:
    - They store the same command
    - The logs are identical in all preceding entries
  - If a given entry is committed, all preceding entries are also committed
 
#### AppendEntries Consistency Check:
  - AppendEntries RPCs include <index, term> of entry preceding new one(s)
  - Follower must contain matching entry; otherwise it rejects request
    - Leader retries with lower log index
  - Implements an induction step, ensures Log matching property

#### Safety: Leader Completeness
  - Once log entry committed, all future leaders must store that entry
  - Servers with incomplete logs must not get elected:
    - Candidates include index and term of last log entry in RequestVote RPCs
    - Voting server denies vote if its log is more up-to-date
    - Logs ranked by <lastTerm, lastIndex>
