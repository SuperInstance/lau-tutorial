# lau-tutorial

An interactive tutorial system for the **Lau** game engine. Teaches git and AI concepts through gameplay — kids think they're playing, but they're actually learning version control, branching, merging, and machine learning fundamentals.

---

## What This Does

`lau-tutorial` provides a step-by-step tutorial framework where each step is a small challenge in the game world. The magic is in the mapping:

| Game action | Real concept |
|---|---|
| "Save my world" | `git commit` |
| "Make a save point" | `git branch` |
| "Keep the changes" | `git merge` |
| "What changed?" | `git diff` |
| "How accurate is my agent?" | Model accuracy evaluation |

The crate ships three built-in tutorials:

1. **Welcome to Your World! 🌍** — Explore, build, save (commit).
2. **Be Brave, Branch Out! 🌿** — Create a branch, experiment, merge or revert.
3. **Agent Whisperer 🤖** — Meet AI agents, teach them by exploring, check their accuracy.

Each step has an action, a completion check, a hint, and a reward badge. Steps that map to git/AI concepts include an optional explanation string (`git_concept`) that can be shown after completion.

---

## Key Idea

The tutorial is a **state machine**. Each `Tutorial` tracks a `current_step` index and a list of completed step IDs. When the game detects that a player has performed an action (built a structure, said a phrase, visited a room, etc.), it calls `try_complete()` with a `CompletionCheck`. If the check matches the current step's expected completion, the step is marked done, the reward is collected, and the tutorial advances.

This is pattern-matching completion — no LLM, no fuzzy matching. The game action system produces a `CompletionCheck` enum variant, and the tutorial compares it structurally.

---

## Install

```toml
[dependencies]
lau-tutorial = "0.1"
```

Requires **Rust 2021 edition**.

| dependency | why |
|---|---|
| `serde` + `serde_json` | Serialize/deserialize tutorials, manager state for save/load |

---

## Quick Start

```rust
use lau_tutorial::*;

// Create the manager and start a tutorial
let mut mgr = TutorialManager::new();
mgr.start("welcome");
println!("hint: {}", mgr.current_hint().unwrap());

// Game loop: player does things, you check completion
// Step 1: wait a few ticks
let reward = mgr.complete_step(&CompletionCheck::TicksPassed { min: 5 });
assert!(reward.unwrap().contains("Explorer"));

// Step 2: player builds a cabin
let reward = mgr.complete_step(&CompletionCheck::BuiltStructure { name: "cabin".into() });

// Step 3: player saves ("commits")
let reward = mgr.complete_step(&CompletionCheck::MadeCommit { message_contains: "cabin".into() });

// Tutorial complete
assert!(!mgr.is_tutorial_active());
println!("rewards earned: {:?}", mgr.total_rewards);
```

---

## API Reference

### `TutorialStep`

| field | type | description |
|---|---|---|
| `id` | `String` | Unique step identifier. |
| `title` | `String` | Display title. |
| `description` | `String` | What the player needs to do. |
| `hint` | `String` | Nudge text shown when the player is stuck. |
| `action` | `TutorialAction` | The kind of action required. |
| `completion` | `CompletionCheck` | The condition that marks this step done. |
| `reward` | `String` | Badge/reward text given on completion. |
| `git_concept` | `Option<String>` | Optional git/AI concept explanation shown after completion. |

### `TutorialAction`

```rust
pub enum TutorialAction {
    Build { structure: String },
    Speak { phrase: String },
    Observe { room: String },
    Save { message: String },       // → git commit
    Branch { name: String },        // → git branch
    Merge { branch: String },       // → git merge
    Teach { agent: String },
    Explore { rooms: usize },
    CheckConservation,
    FreePlay { min_ticks: u64 },
}
```

### `CompletionCheck`

```rust
pub enum CompletionCheck {
    BuiltStructure { name: String },
    SaidPhrase { contains: String },       // substring match
    VisitedRoom { room: String },
    MadeCommit { message_contains: String },
    CreatedBranch { name: String },
    MergedBranch { name: String },
    AgentAccuracy { min: f64 },            // actual >= expected
    VisitedRooms { count: usize },         // actual >= expected
    ConservationError { max: f64 },        // actual <= expected
    TicksPassed { min: u64 },              // actual >= expected
}
```

### `Tutorial`

| method | description |
|---|---|
| `new(id, title, steps)` | Create a tutorial with steps starting at index 0. |
| `current()` → `Option<&TutorialStep>` | The active step, or `None` if completed. |
| `try_complete(check)` → `bool` | If `check` matches current step's completion, advance. |
| `progress()` → `f64` | Fraction complete: `completed_steps / total_steps` (0.0–1.0). |
| `hint()` → `Option<&str>` | Hint text for the current step. |

### `TutorialManager`

| method | description |
|---|---|
| `new()` | Creates a manager pre-loaded with all three built-in tutorials. |
| `start(id)` → `bool` | Activate a tutorial by ID (`"welcome"`, `"branch"`, `"agent"`). |
| `complete_step(check)` → `Option<String>` | Try to complete the active tutorial's current step. Returns reward on success. |
| `current_hint()` → `Option<&str>` | Hint for the active tutorial's current step. |
| `available_tutorials()` → `Vec<&str>` | All tutorial IDs. |
| `is_tutorial_active()` → `bool` | Whether a tutorial is in progress. |

Fields: `tutorials`, `active`, `completed_tutorials`, `total_rewards` — all serialisable.

### Built-in Tutorials

| id | title | steps | teaches |
|---|---|---|---|
| `"welcome"` | Welcome to Your World! 🌍 | 3 | Exploration, building, saving (= commit) |
| `"branch"` | Be Brave, Branch Out! 🌿 | 3 | Branching, experimenting, merging |
| `"agent"` | Agent Whisperer 🤖 | 3 | Meeting agents, teaching via exploration, accuracy |

### `all_tutorials()` → `Vec<Tutorial>`

Returns all three built-in tutorials as a vector.

---

## How It Works

```
┌──────────────────────────────────────────────────────────────┐
│                     TutorialManager                           │
│  tutorials: HashMap<String, Tutorial>                         │
│  active: Option<String>                                       │
│  completed_tutorials: Vec<String>                             │
│  total_rewards: Vec<String>                                   │
└──────────┬───────────────────────────────────────────────────┘
           │ start("welcome")
           ▼
┌──────────────────────────────────────────────────────────────┐
│                       Tutorial                                │
│  current_step: usize                                          │
│  completed_steps: Vec<String>                                 │
│  steps: [Step0, Step1, Step2, ...]                            │
│                                                               │
│  current() ──→ steps[current_step]                            │
│  try_complete(check):                                         │
│    if completion_matches(steps[current_step].completion,      │
│                          check):                              │
│      completed_steps.push(step.id)                            │
│      current_step += 1                                        │
│      if current_step >= steps.len(): completed = true         │
│      return true                                              │
└──────────────────────────────────────────────────────────────┘
```

### Completion Matching Rules

The `completion_matches` function compares the **expected** check (from the step definition) with the **actual** check (from the game):

| expected | actual | match condition |
|---|---|---|
| `BuiltStructure { name }` | `BuiltStructure { name }` | exact equality |
| `SaidPhrase { contains }` | `SaidPhrase { contains }` | actual.contains(expected) |
| `VisitedRoom { room }` | `VisitedRoom { room }` | exact equality |
| `MadeCommit { msg }` | `MadeCommit { msg }` | actual.contains(expected) |
| `CreatedBranch { name }` | `CreatedBranch { name }` | exact equality |
| `MergedBranch { name }` | `MergedBranch { name }` | exact equality |
| `AgentAccuracy { min }` | `AgentAccuracy { min }` | actual ≥ expected |
| `VisitedRooms { count }` | `VisitedRooms { count }` | actual ≥ expected |
| `ConservationError { max }` | `ConservationError { max }` | actual ≤ expected |
| `TicksPassed { min }` | `TicksPassed { min }` | actual ≥ expected |
| *different variants* | | always false |

Threshold-type checks (`AgentAccuracy`, `VisitedRooms`, `ConservationError`, `TicksPassed`) use **≥/≤** comparison — exceeding the requirement counts as completion.

---

## The Math

### Progress Fraction

$$P = \frac{|\text{completed\_steps}|}{|\text{steps}|}$$

Returns 1.0 for an empty tutorial (division by zero guard).

### Step Completion as Pattern Matching

Each step defines a predicate $f(\text{actual}) \to \text{bool}$:

- **Equality**: $f(x) = [x = \text{expected}]$ (e.g. `BuiltStructure`, `VisitedRoom`)
- **Substring**: $f(x) = [\text{expected} \subseteq x]$ (e.g. `SaidPhrase`, `MadeCommit`)
- **Threshold**: $f(x) = [x \geq \text{min}]$ or $f(x) = [x \leq \text{max}]$ (e.g. `AgentAccuracy`, `TicksPassed`)

The tutorial is a **sequential state machine** — steps must be completed in order. The `current_step` index only advances when a match succeeds.

### Tutorial Completion

A tutorial is complete when:

$$\text{current\_step} \geq |\text{steps}|$$

At that point, `completed = true` and `current()` returns `None`.

---

## Tests

**19 tests** covering:

- All three tutorial definitions (step count, titles, git concepts)
- Progress tracking (0%, partial, 100%)
- Step completion with correct checks
- Rejection of wrong checks
- Full tutorial walkthrough
- Hint retrieval (active and completed)
- TutorialManager: start, wrong ID, step completion, full completion
- Git concept content verification
- Available tutorials count
- Completion matching (phrase substring, accuracy threshold)
- Serde JSON round-trip for TutorialManager

```bash
cargo test
```

---

## License

MIT
