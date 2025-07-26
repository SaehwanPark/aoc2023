---
title: "Explaining Day 25 Strategy"
author: "Sae-Hwan Park"
---

# AOC 2023 Day 25: Snowverload

## Problem

Given an undirected graph representing connected components (weather machines), find exactly **3 edges** to remove that will split the graph into two disconnected components. Return the product of the sizes of these two components.

This is a **minimum cut problem** where we know the cut size is exactly 3.

## Possible Approaches

### 1. **Brute Force** 
Try all combinations of 3 edges to remove.
- **Complexity**: O(E³) where E is number of edges
- **Feasibility**: Impractical for large graphs

### 2. **Max-Flow Min-Cut** 
Use Ford-Fulkerson or similar algorithms.
- **Challenge**: Requires knowing source/sink vertices
- **Complexity**: O(VE²) with good implementations

### 3. **Karger's Randomized Algorithm** 
Repeatedly contract random edges until 2 super-nodes remain.
- **Advantage**: No need to specify source/sink
- **Complexity**: O(V²) per iteration, high success probability

## Algorithm Choice: Karger's Algorithm

**Why Karger's works for this problem:**

1. **Probabilistic guarantee**: Each run has probability ≥ 2/(n(n-1)) of finding the minimum cut
2. **Perfect fit**: We know the minimum cut is exactly 3, so we can verify success
3. **Simple implementation**: Just random edge contractions + verification

**Core mechanism:**
- **Edge contraction**: Merge two adjacent nodes into a "super-node"
- **Self-loop removal**: Eliminate edges between the merged nodes
- **Reference updating**: All other nodes now point to the new super-node
- **Termination**: Stop when only 2 super-nodes remain

**Why it preserves minimum cut:**
- Edges within the same final component get contracted away
- Edges crossing the cut boundary are more likely to survive
- With enough iterations, we'll eventually find the minimum cut

## Rust Implementation Notes

### Non-trivial Translation Aspects

**1. Random Selection from HashMap Keys**
```rust
// Challenge: HashMap keys don't implement SliceRandom directly
let keys: Vec<_> = graph.keys().cloned().collect();
let node_a = keys.choose(&mut rng).unwrap().clone();
```

**2. Graph Node Merging**
```rust
fn merge_nodes(&self, graph: &mut Graph, old_node: &str, new_node: &str) {
  if let Some(neighbors) = graph.remove(old_node) {
    for target in neighbors {
      if let Some(target_neighbors) = graph.get_mut(&target) {
        // Update all references to old_node → new_node
        for neighbor in target_neighbors.iter_mut() {
          if neighbor == old_node {
            *neighbor = new_node.to_string();
          }
        }
      }
    }
  }
}
```

**3. Ownership & Borrowing Management**
- **Cloning strategy**: Explicit `.clone()` calls where ownership transfer needed
- **Mutable borrowing**: Careful `&mut` usage for in-place graph modifications
- **Entry API**: Using `HashMap::entry().or_insert_with()` for idiomatic insertions

**4. Modern Rust Patterns**
```rust
// Using rand::prelude::* for trait imports
use rand::prelude::*;

// Proper error propagation
fn main() -> Result<()> {
  let input = read_file("input/d25_full.txt")?;
  // ...
}
```

### Algorithm Verification
```rust
if graph[node_a].len() == 3 {
  return counts[node_a] * counts[node_b];
}
```

The algorithm only returns when exactly 3 edges connect the final two super-nodes, confirming we found the minimum cut of size 3.

**Expected runtime**: Usually finds the answer within a few iterations, but theoretically could run indefinitely (extremely low probability).