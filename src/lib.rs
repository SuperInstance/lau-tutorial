//! Lau Tutorial — teaches git and AI concepts through gameplay.
//!
//! Kids don't know they're learning git. They think they're playing a game.
//! "Save my world" = git commit. "Try something crazy" = git branch.
//! "Keep the changes" = git merge. "What changed?" = git diff.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A tutorial step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialStep {
    pub id: String,
    pub title: String,
    pub description: String,
    pub hint: String,
    pub action: TutorialAction,
    pub completion: CompletionCheck,
    pub reward: String,
    pub git_concept: Option<String>,
}

/// What the kid needs to do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TutorialAction {
    Build { structure: String },
    Speak { phrase: String },
    Observe { room: String },
    Save { message: String },
    Branch { name: String },
    Merge { branch: String },
    Teach { agent: String },
    Explore { rooms: usize },
    CheckConservation,
    FreePlay { min_ticks: u64 },
}

/// How we check completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionCheck {
    BuiltStructure { name: String },
    SaidPhrase { contains: String },
    VisitedRoom { room: String },
    MadeCommit { message_contains: String },
    CreatedBranch { name: String },
    MergedBranch { name: String },
    AgentAccuracy { min: f64 },
    VisitedRooms { count: usize },
    ConservationError { max: f64 },
    TicksPassed { min: u64 },
}

/// A complete tutorial.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutorial {
    pub id: String,
    pub title: String,
    pub steps: Vec<TutorialStep>,
    pub current_step: usize,
    pub completed: bool,
    pub completed_steps: Vec<String>,
    pub started_at_tick: u64,
}

impl Tutorial {
    pub fn new(id: &str, title: &str, steps: Vec<TutorialStep>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            steps,
            current_step: 0,
            completed: false,
            completed_steps: Vec::new(),
            started_at_tick: 0,
        }
    }

    /// Current step (if any).
    pub fn current(&self) -> Option<&TutorialStep> {
        if self.completed { return None; }
        self.steps.get(self.current_step)
    }

    /// Try to complete the current step.
    pub fn try_complete(&mut self, check: &CompletionCheck) -> bool {
        if self.completed { return false; }
        if let Some(step) = self.steps.get(self.current_step) {
            if completion_matches(&step.completion, check) {
                self.completed_steps.push(step.id.clone());
                self.current_step += 1;
                if self.current_step >= self.steps.len() {
                    self.completed = true;
                }
                return true;
            }
        }
        false
    }

    /// Progress as fraction 0.0-1.0.
    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() { return 1.0; }
        self.completed_steps.len() as f64 / self.steps.len() as f64
    }

    /// Get the hint for the current step.
    pub fn hint(&self) -> Option<&str> {
        self.current().map(|s| s.hint.as_str())
    }
}

fn completion_matches(expected: &CompletionCheck, actual: &CompletionCheck) -> bool {
    match (expected, actual) {
        (CompletionCheck::BuiltStructure { name: a }, CompletionCheck::BuiltStructure { name: b }) => a == b,
        (CompletionCheck::SaidPhrase { contains: a }, CompletionCheck::SaidPhrase { contains: b }) => b.contains(a),
        (CompletionCheck::VisitedRoom { room: a }, CompletionCheck::VisitedRoom { room: b }) => a == b,
        (CompletionCheck::MadeCommit { message_contains: a }, CompletionCheck::MadeCommit { message_contains: b }) => b.contains(a),
        (CompletionCheck::CreatedBranch { name: a }, CompletionCheck::CreatedBranch { name: b }) => a == b,
        (CompletionCheck::MergedBranch { name: a }, CompletionCheck::MergedBranch { name: b }) => a == b,
        (CompletionCheck::AgentAccuracy { min: a }, CompletionCheck::AgentAccuracy { min: b }) => b >= a,
        (CompletionCheck::VisitedRooms { count: a }, CompletionCheck::VisitedRooms { count: b }) => b >= a,
        (CompletionCheck::ConservationError { max: a }, CompletionCheck::ConservationError { max: b }) => b <= a,
        (CompletionCheck::TicksPassed { min: a }, CompletionCheck::TicksPassed { min: b }) => b >= a,
        _ => false,
    }
}

/// The tutorial "Welcome to Your World" — teaches building and saving.
pub fn tutorial_welcome() -> Tutorial {
    Tutorial::new("welcome", "Welcome to Your World! 🌍", vec![
        TutorialStep {
            id: "look-around".into(),
            title: "Look Around".into(),
            description: "You're in a brand new world! Let's explore.".into(),
            hint: "Say 'what's here?' to look around.".into(),
            action: TutorialAction::FreePlay { min_ticks: 5 },
            completion: CompletionCheck::TicksPassed { min: 5 },
            reward: "Explorer Eyes 👀".into(),
            git_concept: None,
        },
        TutorialStep {
            id: "first-build".into(),
            title: "Build Your First Thing".into(),
            description: "Let's build something! Try saying what you want.".into(),
            hint: "Say 'build a cabin' to create your first structure!".into(),
            action: TutorialAction::Build { structure: "cabin".into() },
            completion: CompletionCheck::BuiltStructure { name: "cabin".into() },
            reward: "First Block 🧱".into(),
            git_concept: None,
        },
        TutorialStep {
            id: "save-world".into(),
            title: "Save Your World".into(),
            description: "Good work! Now let's save so you don't lose it.".into(),
            hint: "Say 'save my world' to keep your creation safe.".into(),
            action: TutorialAction::Save { message: "my first cabin".into() },
            completion: CompletionCheck::MadeCommit { message_contains: "cabin".into() },
            reward: "Saver Badge 💾".into(),
            git_concept: Some("git commit — saves a snapshot of your world".into()),
        },
    ])
}

/// The tutorial "Be Brave, Branch Out" — teaches branching.
pub fn tutorial_branch() -> Tutorial {
    Tutorial::new("branch", "Be Brave, Branch Out! 🌿", vec![
        TutorialStep {
            id: "save-point".into(),
            title: "Create a Save Point".into(),
            description: "Before trying something risky, make a save point!".into(),
            hint: "Say 'make a save point called risky' to protect your world.".into(),
            action: TutorialAction::Branch { name: "risky".into() },
            completion: CompletionCheck::CreatedBranch { name: "risky".into() },
            reward: "Branch Badge 🌿".into(),
            git_concept: Some("git branch — creates a parallel timeline where you can experiment safely".into()),
        },
        TutorialStep {
            id: "go-crazy".into(),
            title: "Try Something Crazy!".into(),
            description: "Your world is safe! Now try building something wild.".into(),
            hint: "Build anything! If it doesn't work out, you can go back.".into(),
            action: TutorialAction::Build { structure: "anything".into() },
            completion: CompletionCheck::BuiltStructure { name: "anything".into() },
            reward: "Daredevil ⚡".into(),
            git_concept: None,
        },
        TutorialStep {
            id: "keep-or-revert".into(),
            title: "Keep the Changes?".into(),
            description: "Did your experiment work? Keep it or go back to your save point!".into(),
            hint: "Say 'keep the changes' if you like it, or 'go back' to undo.".into(),
            action: TutorialAction::Merge { branch: "risky".into() },
            completion: CompletionCheck::MergedBranch { name: "risky".into() },
            reward: "Merge Master 🔀".into(),
            git_concept: Some("git merge — brings your experimental changes into your main world".into()),
        },
    ])
}

/// The tutorial "Agent Whisperer" — teaches AI concepts.
pub fn tutorial_agent() -> Tutorial {
    Tutorial::new("agent", "Agent Whisperer 🤖", vec![
        TutorialStep {
            id: "meet-agent".into(),
            title: "Meet Your Agent".into(),
            description: "Every room has an AI agent that learns from you!".into(),
            hint: "Say 'hello Sparky' to meet the agent in this room.".into(),
            action: TutorialAction::Speak { phrase: "hello".into() },
            completion: CompletionCheck::SaidPhrase { contains: "hello".into() },
            reward: "Friend Finder 🤝".into(),
            git_concept: None,
        },
        TutorialStep {
            id: "teach-agent".into(),
            title: "Teach Your Agent".into(),
            description: "Agents learn by watching. Visit rooms and they'll learn to predict!".into(),
            hint: "Visit different rooms and observe them. Your agent watches and learns.".into(),
            action: TutorialAction::Explore { rooms: 3 },
            completion: CompletionCheck::VisitedRooms { count: 3 },
            reward: "Teacher 📚".into(),
            git_concept: Some("machine learning — agents learn patterns from observations".into()),
        },
        TutorialStep {
            id: "check-accuracy".into(),
            title: "How Smart Is Your Agent?".into(),
            description: "Let's check if your agent learned to predict correctly!".into(),
            hint: "Say 'how accurate is my agent?' to see its score.".into(),
            action: TutorialAction::Teach { agent: "sparky".into() },
            completion: CompletionCheck::AgentAccuracy { min: 0.5 },
            reward: "Agent Trainer 🎓".into(),
            git_concept: Some("accuracy — how well the agent's predictions match reality".into()),
        },
    ])
}

/// All available tutorials.
pub fn all_tutorials() -> Vec<Tutorial> {
    vec![tutorial_welcome(), tutorial_branch(), tutorial_agent()]
}

/// The tutorial manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialManager {
    pub tutorials: HashMap<String, Tutorial>,
    pub active: Option<String>,
    pub completed_tutorials: Vec<String>,
    pub total_rewards: Vec<String>,
}

impl TutorialManager {
    pub fn new() -> Self {
        let mut tutorials = HashMap::new();
        for t in all_tutorials() {
            tutorials.insert(t.id.clone(), t);
        }
        Self { tutorials, active: None, completed_tutorials: Vec::new(), total_rewards: Vec::new() }
    }

    pub fn start(&mut self, id: &str) -> bool {
        if self.tutorials.contains_key(id) {
            self.active = Some(id.into());
            true
        } else { false }
    }

    pub fn complete_step(&mut self, check: &CompletionCheck) -> Option<String> {
        let active_id = self.active.clone()?;
        let tutorial = self.tutorials.get_mut(&active_id)?;
        let step = tutorial.current()?;
        let reward = step.reward.clone();
        if tutorial.try_complete(check) {
            self.total_rewards.push(reward.clone());
            if tutorial.completed {
                self.completed_tutorials.push(active_id);
                self.active = None;
            }
            Some(reward)
        } else { None }
    }

    pub fn current_hint(&self) -> Option<&str> {
        let active_id = self.active.as_ref()?;
        self.tutorials.get(active_id)?.hint()
    }

    pub fn available_tutorials(&self) -> Vec<&str> {
        self.tutorials.keys().map(|s| s.as_str()).collect()
    }

    pub fn is_tutorial_active(&self) -> bool { self.active.is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_tutorial() {
        let t = tutorial_welcome();
        assert_eq!(t.steps.len(), 3);
        assert_eq!(t.title, "Welcome to Your World! 🌍");
    }

    #[test]
    fn test_branch_tutorial() {
        let t = tutorial_branch();
        assert_eq!(t.steps.len(), 3);
        assert!(t.steps[0].git_concept.is_some());
    }

    #[test]
    fn test_agent_tutorial() {
        let t = tutorial_agent();
        assert_eq!(t.steps.len(), 3);
    }

    #[test]
    fn test_all_tutorials() {
        assert_eq!(all_tutorials().len(), 3);
    }

    #[test]
    fn test_progress_zero() {
        let t = tutorial_welcome();
        assert!((t.progress() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_step_completion() {
        let mut t = tutorial_welcome();
        assert!(t.try_complete(&CompletionCheck::TicksPassed { min: 5 }));
        assert_eq!(t.completed_steps.len(), 1);
        assert!((t.progress() - (1.0/3.0)).abs() < 1e-10);
    }

    #[test]
    fn test_wrong_completion() {
        let mut t = tutorial_welcome();
        assert!(!t.try_complete(&CompletionCheck::BuiltStructure { name: "cabin".into() }));
        assert_eq!(t.completed_steps.len(), 0);
    }

    #[test]
    fn test_full_completion() {
        let mut t = tutorial_welcome();
        t.try_complete(&CompletionCheck::TicksPassed { min: 5 });
        t.try_complete(&CompletionCheck::BuiltStructure { name: "cabin".into() });
        t.try_complete(&CompletionCheck::MadeCommit { message_contains: "cabin".into() });
        assert!(t.completed);
        assert!((t.progress() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_hint() {
        let t = tutorial_welcome();
        assert!(t.hint().unwrap().contains("what's here"));
    }

    #[test]
    fn test_no_hint_when_complete() {
        let mut t = tutorial_welcome();
        t.try_complete(&CompletionCheck::TicksPassed { min: 5 });
        t.try_complete(&CompletionCheck::BuiltStructure { name: "cabin".into() });
        t.try_complete(&CompletionCheck::MadeCommit { message_contains: "cabin".into() });
        assert!(t.hint().is_none());
    }

    #[test]
    fn test_manager_start() {
        let mut m = TutorialManager::new();
        assert!(m.start("welcome"));
        assert!(m.is_tutorial_active());
    }

    #[test]
    fn test_manager_wrong_id() {
        let mut m = TutorialManager::new();
        assert!(!m.start("nonexistent"));
    }

    #[test]
    fn test_manager_complete_step() {
        let mut m = TutorialManager::new();
        m.start("welcome");
        let reward = m.complete_step(&CompletionCheck::TicksPassed { min: 5 });
        assert!(reward.is_some());
        assert!(reward.unwrap().contains("Explorer"));
    }

    #[test]
    fn test_manager_full_tutorial() {
        let mut m = TutorialManager::new();
        m.start("welcome");
        m.complete_step(&CompletionCheck::TicksPassed { min: 5 });
        m.complete_step(&CompletionCheck::BuiltStructure { name: "cabin".into() });
        m.complete_step(&CompletionCheck::MadeCommit { message_contains: "cabin".into() });
        assert!(!m.is_tutorial_active());
        assert!(m.completed_tutorials.contains(&"welcome".into()));
        assert_eq!(m.total_rewards.len(), 3);
    }

    #[test]
    fn test_git_concepts() {
        let branch = tutorial_branch();
        assert!(branch.steps[0].git_concept.as_ref().unwrap().contains("branch"));
        assert!(branch.steps[2].git_concept.as_ref().unwrap().contains("merge"));
    }

    #[test]
    fn test_available_tutorials() {
        let m = TutorialManager::new();
        assert_eq!(m.available_tutorials().len(), 3);
    }

    #[test]
    fn test_completion_matches_phrase() {
        let expected = CompletionCheck::SaidPhrase { contains: "hello".into() };
        let actual = CompletionCheck::SaidPhrase { contains: "hello world".into() };
        assert!(completion_matches(&expected, &actual));
    }

    #[test]
    fn test_completion_accuracy() {
        let expected = CompletionCheck::AgentAccuracy { min: 0.5 };
        let actual = CompletionCheck::AgentAccuracy { min: 0.7 };
        assert!(completion_matches(&expected, &actual));
    }

    #[test]
    fn test_serialization() {
        let mut m = TutorialManager::new();
        m.start("welcome");
        let json = serde_json::to_string(&m).unwrap();
        let restored: TutorialManager = serde_json::from_str(&json).unwrap();
        assert!(restored.is_tutorial_active());
        assert_eq!(restored.tutorials.len(), 3);
    }
}
