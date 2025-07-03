/// Example demonstrating the complete XP-Net + Extropy Engine integration
/// Shows how base-10 timekeeping, physics-based XP minting, and retroactive bonuses work together

use common::{
    XpTime, TemporalLoop, LoopType, LoopStatus, 
    PhysicsXpCalculator, XpCalculationInput, XpFormulaType,
    CausalLoop, CausalLoopType, Platform, RetroactiveXpEngine,
    Address, Hash, EntropyReduction, EntropyMeasurement, MeasurementMethod
};
use dag_core::{Transaction, TransactionType, TransactionMetadata, Dag};
use std::collections::HashMap;

/// Comprehensive example of the Extropy Engine integration
pub fn extropy_integration_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 XP-Net + Extropy Engine Integration Example");
    println!("=================================================");

    // 1. Initialize the base-10 temporal system
    demo_temporal_system();

    // 2. Demonstrate physics-based XP calculations
    demo_physics_xp_calculation()?;

    // 3. Show temporal loops and causal closure
    demo_temporal_loops()?;

    // 4. Demonstrate retroactive XP minting
    demo_retroactive_xp_minting()?;

    // 5. Create a complete DAG with entropy-anchored transactions
    demo_entropy_anchored_dag()?;

    // 6. Show cross-platform integration scenario
    demo_cross_platform_scenario()?;

    println!("\n✅ Integration example completed successfully!");
    println!("This demonstrates how XP-Net provides the infrastructure for");
    println!("physics-based governance where truth and coherence become the only currencies.");

    Ok(())
}

/// Demonstrate the base-10 temporal system
fn demo_temporal_system() {
    println!("\n📅 Base-10 Temporal System Demo");
    println!("--------------------------------");

    // Current time in XP format
    let current_xp_time = XpTime::now();
    println!("Current XP Time: {}", current_xp_time);
    println!("  Year: {}", current_xp_time.year);
    println!("  Day of year: {} (0-359)", current_xp_time.day);
    println!("  Hour: {} (0-9)", current_xp_time.hour);
    println!("  Minute: {} (0-99)", current_xp_time.minute);

    // Temporal calculations
    println!("  Week: {} (5-day weeks)", current_xp_time.week());
    println!("  Day of week: {} (0-4)", current_xp_time.day_of_week());
    println!("  Month: {} (15-day months)", current_xp_time.month());
    println!("  Day of month: {} (0-14)", current_xp_time.day_of_month());

    // Demonstrate time arithmetic
    let future_time = current_xp_time.add_minutes(150); // Add 1.5 XP hours
    println!("Future time (+150 minutes): {}", future_time);

    // Convert to/from Gregorian
    let gregorian_timestamp = current_xp_time.to_gregorian_timestamp();
    let converted_back = XpTime::from_gregorian_timestamp(gregorian_timestamp);
    println!("Gregorian timestamp: {}", gregorian_timestamp);
    println!("Converted back: {}", converted_back);
}

/// Demonstrate physics-based XP calculations
fn demo_physics_xp_calculation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚛️  Physics-Based XP Calculation Demo");
    println!("-------------------------------------");

    let calculator = PhysicsXpCalculator::default();

    // Create a comprehensive XP calculation input
    let calculation_input = XpCalculationInput {
        reputation: 0.85, // High reputation contributor
        feedback_closure: 0.92, // Strong feedback loop closure
        entropy_reduction: 3.2, // Significant entropy reduction in J/K
        domain_vector: vec![0.8, 0.1, 0.3, 0.0, 0.9, 0.2], // Cognitive + Creative focus
        essentiality_vector: vec![0.9, 0.1, 0.4, 0.0, 0.8, 0.3], // High importance
        temporal_sustainability: 0.75, // Good long-term stability
        closure_speed: 1.8, // Reasonable causal closure speed
        power_consumption: Some(150.0), // 150 watts
        temperature: Some(298.15), // Room temperature (Kelvin)
        network_impact: 1.3, // Positive network effects
        collaboration_factor: 1.15, // Collaborative work bonus
        calculation_time: XpTime::now(),
        validator: Some(Address::new([1; 20])),
    };

    println!("Input parameters:");
    println!("  Reputation: {:.2}", calculation_input.reputation);
    println!("  Feedback closure: {:.2}", calculation_input.feedback_closure);
    println!("  Entropy reduction: {:.2} J/K", calculation_input.entropy_reduction);
    println!("  Temporal sustainability: {:.2}", calculation_input.temporal_sustainability);
    println!("  Closure speed: {:.2} events/sec", calculation_input.closure_speed);

    // Calculate XP using the complete physics formula
    let result = calculator.calculate_xp(&calculation_input)?;

    println!("\nCalculation result:");
    println!("  XP amount: {} tokens", result.xp_amount);
    println!("  Formula type: {:?}", result.metadata.formula_type);
    println!("  Confidence: {:.2}", result.metadata.confidence);
    println!("  Thermodynamic compliance: {}", result.thermodynamic_compliance.is_compliant);

    println!("\nFormula components:");
    println!("  Reputation factor (R): {:.3}", result.components.reputation_factor);
    println!("  Feedback factor (F): {:.3}", result.components.feedback_factor);
    println!("  Entropy delta (ΔS): {:.3}", result.components.entropy_delta);
    println!("  Domain-essentiality (w·E): {:.3}", result.components.domain_essentiality);
    println!("  Temporal factor log(1/Tₛ): {:.3}", result.components.temporal_factor);
    println!("  Final scaled value: {:.3}", result.components.scaled_value);

    if !result.metadata.cross_domain_synergies.is_empty() {
        println!("  Cross-domain synergies: {:?}", result.metadata.cross_domain_synergies);
    }

    Ok(())
}

/// Demonstrate temporal loops and causal closure
fn demo_temporal_loops() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 Temporal Loops and Causal Closure Demo");
    println!("------------------------------------------");

    // Create a daily optimization loop
    let start_time = XpTime::now();
    let mut daily_loop = TemporalLoop::new(LoopType::Daily, start_time, 5.2);

    println!("Created daily loop:");
    println!("  Loop ID: {}", daily_loop.id);
    println!("  Type: {:?}", daily_loop.loop_type);
    println!("  Start time: {}", daily_loop.start_time);
    println!("  Initial entropy: {:.2} J/K", daily_loop.initial_entropy);

    // Add activities to the loop
    use common::{TemporalActivity};
    
    let activity1 = TemporalActivity {
        id: Hash::new([1; 32]),
        name: "Morning planning session".to_string(),
        start_time: start_time.add_minutes(30),
        duration: 60, // 1 XP hour
        entropy_delta: -0.8, // Reduces entropy
        metadata: serde_json::json!({
            "complexity": 0.7,
            "tool_used": "XP Planner"
        }),
    };

    let activity2 = TemporalActivity {
        id: Hash::new([2; 32]),
        name: "Skill practice session".to_string(),
        start_time: start_time.add_minutes(200),
        duration: 120, // 2 XP hours
        entropy_delta: -1.1, // Further entropy reduction
        metadata: serde_json::json!({
            "skill": "Quantum Computing",
            "mastery_gain": 0.15
        }),
    };

    daily_loop.add_activity(activity1);
    daily_loop.add_activity(activity2);

    println!("Added activities:");
    println!("  Activity 1: {} (Δ entropy: {:.2})", daily_loop.activities[0].name, daily_loop.activities[0].entropy_delta);
    println!("  Activity 2: {} (Δ entropy: {:.2})", daily_loop.activities[1].name, daily_loop.activities[1].entropy_delta);

    // Close the loop
    let end_time = start_time.add_minutes(600); // 6 XP hours later
    let final_entropy = 3.1; // Entropy reduced from 5.2 to 3.1
    let entropy_reduction = daily_loop.close(end_time, final_entropy)?;

    println!("\nLoop closure:");
    println!("  End time: {}", end_time);
    println!("  Final entropy: {:.2} J/K", final_entropy);
    println!("  Total entropy reduction: {:.2} J/K", entropy_reduction);
    println!("  Loop status: {:?}", daily_loop.status);
    println!("  Duration: {} XP minutes", daily_loop.duration_minutes().unwrap());

    Ok(())
}

/// Demonstrate retroactive XP minting for causal loops
fn demo_retroactive_xp_minting() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💰 Retroactive XP Minting Demo");
    println!("-------------------------------");

    let engine = RetroactiveXpEngine::default();

    // Create a cross-platform causal loop
    let user_address = Address::new([42; 20]);
    let mut causal_loop = CausalLoop::new(
        user_address,
        CausalLoopType::CrossPlatform,
        Platform::Unified,
        6.5, // Initial entropy
    );

    println!("Created cross-platform causal loop:");
    println!("  Loop ID: {}", causal_loop.id);
    println!("  Type: {:?}", causal_loop.loop_type);
    println!("  Platform: {:?}", causal_loop.platform);
    println!("  Initial entropy: {:.2} J/K", causal_loop.initial_entropy);

    // Add events to the causal chain
    use common::{CausalEvent, CausalEventType};

    let events = vec![
        CausalEvent {
            id: Hash::new([10; 32]),
            event_type: CausalEventType::Contribution,
            platform: Platform::Academy,
            event_data: serde_json::json!({
                "type": "skill_development",
                "skill": "Quantum Error Correction",
                "mastery_level": 0.8
            }),
            entropy_delta: -1.2,
            timestamp: XpTime::now(),
            processed: false,
            retroactive_xp: 0.0,
        },
        CausalEvent {
            id: Hash::new([11; 32]),
            event_type: CausalEventType::SkillEmergence,
            platform: Platform::SignalFlow,
            event_data: serde_json::json!({
                "emerged_skill": "Quantum-AI Integration",
                "coherence_score": 0.85
            }),
            entropy_delta: -0.9,
            timestamp: XpTime::now().add_minutes(180),
            processed: false,
            retroactive_xp: 0.0,
        },
        CausalEvent {
            id: Hash::new([12; 32]),
            event_type: CausalEventType::ValidationConsensus,
            platform: Platform::Unified,
            event_data: serde_json::json!({
                "consensus_score": 0.92,
                "validator_count": 7
            }),
            entropy_delta: -0.4,
            timestamp: XpTime::now().add_minutes(240),
            processed: false,
            retroactive_xp: 0.0,
        },
    ];

    println!("\nAdding causal events:");
    for (i, event) in events.iter().enumerate() {
        causal_loop.add_event(event.clone());
        println!("  Event {}: {:?} on {:?} (Δ entropy: {:.2})", 
                 i + 1, event.event_type, event.platform, event.entropy_delta);
    }

    // Detect loop closure
    let last_event = events.last().unwrap().clone();
    let loop_closed = engine.detect_loop_closure(&mut causal_loop, last_event)?;

    if loop_closed {
        println!("\n🎉 Loop closure detected!");
        println!("  Loop state: {:?}", causal_loop.loop_state);
        println!("  Final entropy: {:.2} J/K", causal_loop.final_entropy.unwrap());
        println!("  Total entropy reduction: {:.2} J/K", causal_loop.entropy_reduction.unwrap());
        
        // Calculate retroactive XP
        let retroactive_xp = engine.calculate_retroactive_xp(&causal_loop)?;
        println!("  Retroactive XP calculated: {:.2}", retroactive_xp);

        // Create retroactive transaction
        let original_xp = 150.0; // Original XP earned
        let retro_transaction = engine.create_retroactive_transaction(&causal_loop, original_xp)?;
        
        println!("\nRetroactive transaction created:");
        println!("  Transaction ID: {}", retro_transaction.id);
        println!("  User: {}", retro_transaction.user_id);
        println!("  Original XP: {:.2}", retro_transaction.original_xp);
        println!("  Retroactive XP: {:.2}", retro_transaction.retroactive_xp);
        println!("  Bonus reason: {}", retro_transaction.bonus_reason);
        println!("  Multiplier: {:.2}", retro_transaction.multiplier);
        println!("  Platform origin: {:?}", retro_transaction.platform_origin);
    }

    Ok(())
}

/// Demonstrate entropy-anchored DAG with physics validation
fn demo_entropy_anchored_dag() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌐 Entropy-Anchored DAG Demo");
    println!("----------------------------");

    let mut dag = Dag::new();

    // Create genesis transaction
    let genesis_tx = Transaction::new(
        TransactionType::Transfer {
            from: Address::new([0; 20]),
            to: Address::new([1; 20]),
            amount: common::XpAmount::new(1000000000000000000000, 0.0), // 1000 XP
        },
        vec![], // No parents for genesis
        0,
        TransactionMetadata::default(),
    );

    println!("Adding genesis transaction: {}", genesis_tx.hash);
    dag.add_genesis_transaction(genesis_tx.clone())?;

    // Create entropy reduction claim
    let entropy_reduction = EntropyReduction {
        initial: EntropyMeasurement {
            value: 8.5,
            confidence: 0.95,
            domain: "quantum_computing".to_string(),
            method: MeasurementMethod::Computational { operations_saved: 1000000 },
        },
        final_state: EntropyMeasurement {
            value: 5.8,
            confidence: 0.93,
            domain: "quantum_computing".to_string(),
            method: MeasurementMethod::Computational { operations_saved: 1000000 },
        },
        delta: 2.7, // Significant entropy reduction
        duration_ns: 3600000000000, // 1 hour
        validators: vec![], // Would be populated with actual validators
    };

    let entropy_tx = Transaction::new(
        TransactionType::EntropyReductionClaim {
            claimant: Address::new([1; 20]),
            reduction: entropy_reduction,
        },
        vec![genesis_tx.hash],
        1,
        TransactionMetadata::default(),
    );

    println!("Adding entropy reduction claim: {}", entropy_tx.hash);
    dag.add_transaction(entropy_tx.clone())?;

    // Create physics-based XP mint
    let physics_input = XpCalculationInput {
        reputation: 0.78,
        feedback_closure: 0.88,
        entropy_reduction: 2.1,
        domain_vector: vec![0.9, 0.1, 0.0, 0.0, 0.0, 0.7], // Cognitive + System
        essentiality_vector: vec![0.85, 0.15, 0.0, 0.0, 0.0, 0.8],
        temporal_sustainability: 0.82,
        closure_speed: 1.5,
        power_consumption: Some(120.0),
        temperature: Some(295.0),
        network_impact: 1.25,
        collaboration_factor: 1.0,
        calculation_time: XpTime::now(),
        validator: Some(Address::new([2; 20])),
    };

    let physics_tx = Transaction::new(
        TransactionType::PhysicsXpMint {
            recipient: Address::new([1; 20]),
            calculation_input: physics_input,
        },
        vec![entropy_tx.hash],
        2,
        TransactionMetadata::default(),
    );

    println!("Adding physics XP mint: {}", physics_tx.hash);
    dag.add_transaction(physics_tx.clone())?;

    // Display DAG statistics
    println!("\nDAG Statistics:");
    println!("  Total transactions: {}", dag.transactions.len());
    println!("  Current tips: {:?}", dag.get_tips());
    
    let account_state = dag.get_account_state(&Address::new([1; 20]));
    println!("  Account balance: {} XP tokens", account_state.balance);
    println!("  Total entropy contributed: {:.2} J/K", account_state.entropy_contributed);
    println!("  Reputation score: {:.2}", account_state.reputation_score);

    // Demonstrate topological ordering
    let topo_order = dag.topological_sort()?;
    println!("  Topological order: {} transactions", topo_order.len());

    Ok(())
}

/// Demonstrate cross-platform integration scenario
fn demo_cross_platform_scenario() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔗 Cross-Platform Integration Scenario");
    println!("---------------------------------------");

    println!("Scenario: A user develops quantum computing skills across multiple platforms");
    println!("and receives retroactive bonuses when cross-platform synergies are detected.");

    // Simulated sequence of events across platforms
    let user = Address::new([99; 20]);
    let events = vec![
        ("LevelUp Academy", "Quantum algorithm mastery", 45.0, -1.8),
        ("SignalFlow", "AI-quantum integration task", 32.0, -1.2),
        ("XP Timekeeping", "Temporal optimization loop", 28.0, -0.9),
        ("Merchant Network", "Quantum consulting service", 67.0, -2.1),
        ("Validator Mesh", "Quantum validation consensus", 38.0, -1.4),
    ];

    println!("\nSequence of cross-platform activities:");
    let mut total_xp = 0.0;
    let mut total_entropy_reduction = 0.0;

    for (i, (platform, activity, xp, entropy_delta)) in events.iter().enumerate() {
        total_xp += xp;
        total_entropy_reduction += entropy_delta.abs();
        
        println!("  {}. {} on {}: {} XP (Δ entropy: {:.1} J/K)", 
                 i + 1, activity, platform, xp, entropy_delta);
    }

    println!("\nInitial totals:");
    println!("  Total XP earned: {:.1}", total_xp);
    println!("  Total entropy reduction: {:.1} J/K", total_entropy_reduction);

    // Detect cross-platform synergies
    let synergies = vec![
        ("quantum_ai_integration", 1.25),
        ("temporal_optimization", 1.15),
        ("validation_expertise", 1.18),
        ("multi_platform_mastery", 1.35),
    ];

    println!("\nDetected cross-platform synergies:");
    let mut total_multiplier = 1.0;
    for (synergy, multiplier) in &synergies {
        println!("  {}: {:.2}x bonus", synergy, multiplier);
        total_multiplier *= multiplier;
    }

    let retroactive_bonus = total_xp * (total_multiplier - 1.0);
    let final_xp = total_xp + retroactive_bonus;

    println!("\nRetroactive calculation:");
    println!("  Combined multiplier: {:.2}x", total_multiplier);
    println!("  Retroactive bonus: {:.1} XP", retroactive_bonus);
    println!("  Final XP total: {:.1} XP", final_xp);

    // Show how this feeds back into the physics formula
    println!("\nPhysics formula impact:");
    println!("  Higher reputation from cross-platform work");
    println!("  Increased network effects (collaboration factor)");
    println!("  Enhanced domain expertise weighting");
    println!("  Stronger causal loop closure");

    println!("\n🎯 Result: The user's expertise in quantum computing is validated");
    println!("across multiple platforms, creating a positive feedback loop that");
    println!("increases their reputation and enables even higher XP generation");
    println!("in future contributions. This is how coherence becomes currency.");

    Ok(())
}

fn main() {
    if let Err(e) = extropy_integration_example() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}