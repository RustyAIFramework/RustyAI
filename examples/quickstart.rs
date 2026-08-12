//! Quickstart example - Demonstrating the facade works

use rustyai::prelude::*;

fn main() {
    println!("🤖 RustyAI Quickstart Example\n");

    // Actually use types from each crate to prove the facade works
    let agent1 = AgentId::new();
    let agent2 = AgentId::new();
    println!("agent-core: Created agents {} and {}", agent1, agent2);

    let message = Message::new(agent1, agent2, Performative::Inform, "Hello!");
    println!("messaging: Created message: {}", message.content());

    let agent = BDIAgent::new(AgentId::new());
    println!("cognition: BDI agent has {} beliefs", agent.belief_count());

    let swarm = SwarmStructure::new("TestSwarm");
    println!("patterns: Swarm '{}' created", swarm.name());

    let _config = RuntimeConfig::default();
    println!("runtime: RuntimeConfig created");

    println!("\n All 5 crates working through the facade!");
}