<script lang="ts">
	import { invalidateAll } from '$app/navigation'
	import FormGroup from '$components/FormGroup.svelte'
	import Modal from '$components/Modal.svelte'
	import Select from '$components/Select.svelte'
	import { invoke } from '$invoke'
	import { Assignment, AssignmentColorMap, AssignmentDifficulties, AssignmentFrequencies } from '$models/assignment.svelte'
	import { Flame } from 'lucide-svelte'

	interface Props {
		onclose: () => void
		assignment: Assignment
	}

	let { onclose, assignment }: Props = $props()
	let newAssignment = $state(new Assignment(assignment))

	async function editAssignment() {
		try {
			await invoke('update_assignment_command', newAssignment.toUpdateDTO())
			await invalidateAll()
			onclose()
		} catch (error) {
			console.error('Failed to delete assignment:', error)
		}
	}
</script>

<Modal show title="Editar {assignment.name}" onconfirm={editAssignment} {onclose} style={'overflow-y: unset'}>
	<div class="group">
		<FormGroup id="difficulty" label="Dificultad">
			<Select id="difficulty" bind:value={newAssignment.difficulty} required>
				{#each AssignmentDifficulties as { label, value }}
					<option {value}>
						<Flame color="var(--gray-light)" fill={AssignmentColorMap.get(value)} />
						{label}
					</option>
				{/each}
			</Select>
		</FormGroup>
		<FormGroup id="frequency" label="Frecuencia">
			<Select id="frequency" bind:value={newAssignment.frequency} required>
				{#each AssignmentFrequencies as { label, value }}
					<option {value}>{label}</option>
				{/each}
			</Select>
		</FormGroup>
	</div>
</Modal>

<style>
	.group {
		display: flex;
		gap: 1rem;
	}
</style>
