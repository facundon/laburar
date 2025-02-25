<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { Save } from 'lucide-svelte'
	import CongratsText from '$components/CongratsText.svelte'
	import AssignmentCheckbox from '$pages/employees/[id]/assign-task/AssignmentCheckbox.svelte'
	import { SvelteSet } from 'svelte/reactivity'
	import { goto } from '$app/navigation'

	let { data } = $props()
	const employee = data.employee
	let selectedTasks = new SvelteSet<number>()

	let search = $state('')
	let assignments = $derived(
		data.assignments.filter(assignment => {
			const searchValue = search.toLowerCase()
			return assignment.name.toLowerCase().includes(searchValue)
		}),
	)

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
			goto(ROUTES.employee.view(employee.id))
		} catch (error) {
			console.error('Failed to assign task:', error)
		}
	}
</script>

{#if employee}
	<MainContainer title={`Asignar Tareas a ${employee.name}`}>
		<div class="search">
			<input bind:value={search} placeholder="Buscar..." />
		</div>
		{#if data.assignments.length === 0}
			<p><CongratsText>Pero que maravilloso trabajo!</CongratsText> {employee.firstName} ya tiene todas las tareas asignadas</p>
		{:else}
			<form onsubmit={assignTasks}>
				<div class="task-list">
					{#if assignments.length > 0}
						{#each assignments as assignment}
							<AssignmentCheckbox {assignment} onchange={() => toggleTask(assignment.id)} />
						{/each}
					{:else}
						<p>No se encontraron tareas</p>
					{/if}
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
	.search {
		margin-bottom: 1rem;
		width: 100%;
		display: flex;
	}
	input {
		flex-grow: 1;
		padding: 0.5rem;
		border: 1px solid var(--color-gray-300);
		border-radius: 0.25rem;
	}
	.actions {
		display: flex;
		justify-content: space-between;
	}

	.task-list {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 2rem;
		max-height: 450px;
		max-width: min-content;
		overflow-y: auto;
		margin-bottom: 2rem;
		white-space: nowrap;
	}
</style>
