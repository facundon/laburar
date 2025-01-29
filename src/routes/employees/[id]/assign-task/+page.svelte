<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { Save } from 'lucide-svelte'
	import CongratsText from '$components/CongratsText.svelte'
	import AssignmentCheckbox from '$pages/employees/[id]/assign-task/AssignmentCheckbox.svelte'
	import { SvelteSet } from 'svelte/reactivity'

	let { data } = $props()
	const employee = data.employee
	const assignments = data.assignments
	let selectedTasks = new SvelteSet<number>()

	function toggleTask(taskId: number) {
		if (selectedTasks.has(taskId)) selectedTasks.delete(taskId)
		else selectedTasks.add(taskId)
	}

	async function assignTasks(e: Event) {
		e.preventDefault()
		if (!employee) return
		try {
			const assignmentIds = Array.from(selectedTasks)
			await invoke('create_assignments_to_employee_command', { employee_id: employee.id, assignment_ids: assignmentIds })
			window.location.href = ROUTES.employee.view(employee.id)
		} catch (error) {
			console.error('Failed to assign task:', error)
		}
	}
</script>

{#if employee}
	<MainContainer title={`Asignar Tareas a ${employee.name}`}>
		{#if assignments.length === 0}
			<p><CongratsText>Pero que maravilloso trabajo!</CongratsText> {employee.firstName} ya tiene todas las tareas asignadas</p>
		{:else}
			<form onsubmit={assignTasks}>
				<div class="task-list">
					{#each assignments as assignment}
						<AssignmentCheckbox {assignment} onchange={() => toggleTask(assignment.id)} />
					{/each}
				</div>
				<div class="actions">
					<Button outlined variant="secondary" href={ROUTES.employee.view(employee.id)}>Cancelar</Button>
					<Button type="submit" Icon={Save} disabled={selectedTasks.size === 0}>Asignar</Button>
				</div>
			</form>
		{/if}
	</MainContainer>
{/if}

<style>
	.actions {
		display: flex;
		justify-content: space-between;
	}

	.task-list {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 2rem;
		max-height: 450px;
		max-width: min-content;
		overflow-y: auto;
		margin-bottom: 2rem;
		white-space: nowrap;
	}
</style>
