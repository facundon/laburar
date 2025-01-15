<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { Save } from 'lucide-svelte'
	import Select from '$components/Select.svelte'
	import { Assignment, AssignmentDifficulties, AssignmentFrequencies } from '$models/assignment'
	import FormGroup from '$components/FormGroup.svelte'
	import CongratsText from '$components/CongratsText.svelte'

	let { data } = $props()
	const area = data.area
	const tasks = data.tasks

	let assignment = $state(new Assignment({ areaId: area?.id }))

	async function assignTasks() {
		if (!area) return
		try {
			await invoke('create_assignment_command', assignment.toCreateDTO())
			window.location.href = ROUTES.area.view(area.id)
		} catch (error) {
			console.error('Failed to assign task:', error)
		}
	}
</script>

{#if area}
	<MainContainer title={`Asignar Tarea a ${area.name}`}>
		{#if tasks.length === 0}
			<p><CongratsText>Pero que maravilloso trabajo!</CongratsText> Ya asignaste todas las tareas disponibles.</p>
		{:else}
			<form onsubmit={assignTasks}>
				<div class="group">
					<FormGroup id="taskId" label="Tarea">
						<Select id="taskId" bind:value={assignment.taskId} required placeholder="Selecciona una tarea">
							{#each tasks as task}
								<option value={task.id}>
									{task.name}
								</option>
							{/each}
						</Select>
					</FormGroup>
				</div>
				<div class="group">
					<FormGroup id="difficulty" label="Dificultad">
						<Select id="difficulty" bind:value={assignment.difficulty} required>
							{#each AssignmentDifficulties as { label, value }}
								<option {value}>{label}</option>
							{/each}
						</Select>
					</FormGroup>
					<FormGroup id="frequency" label="Frecuencia">
						<Select id="frequency" bind:value={assignment.frequency} required>
							{#each AssignmentFrequencies as { label, value }}
								<option {value}>{label}</option>
							{/each}
						</Select>
					</FormGroup>
				</div>

				<div class="actions">
					<Button outlined variant="secondary" href={ROUTES.area.view(area.id)}>Cancelar</Button>
					<Button type="submit" Icon={Save}>Asignar</Button>
				</div>
			</form>
		{/if}
	</MainContainer>
{/if}

<style>
	.group {
		display: flex;
		gap: 3rem;
	}
	.actions {
		display: flex;
		justify-content: space-between;
	}
</style>
