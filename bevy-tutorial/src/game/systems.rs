use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
	next_sim_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
	next_sim_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
	keyboard_input: Res<Input<KeyCode>>,
	simulation_state: Res<State<SimulationState>>,
	mut next_sim_state: ResMut<NextState<SimulationState>>
) {
	if keyboard_input.just_pressed(KeyCode::Space) {
		if *simulation_state.get() == SimulationState::Running {
			next_sim_state.set(SimulationState::Paused);
			println!("Simulation Paused.");
		} else if *simulation_state.get() == SimulationState::Paused {
			next_sim_state.set(SimulationState::Running);
			println!("Simulation Running.");
		}
	}
}
