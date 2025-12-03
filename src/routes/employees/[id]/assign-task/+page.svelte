<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Accordion from '$components/Accordion.svelte'
	import { Save } from 'lucide-svelte'
	import CongratsText from '$components/CongratsText.svelte'
	import AssignmentCheckbox from '$pages/employees/[id]/assign-task/AssignmentCheckbox.svelte'
	import { SvelteSet } from 'svelte/reactivity'
	import { goto, invalidateAll } from '$app/navigation'
	import type { Assignment } from '$models/assignment.svelte'

	let { data } = $props()
	const employee = data.employee
	let selectedTasks = new SvelteSet<number>()

	let search = $state('')

	// Filter assignments by search term first
	let filteredAssignments = $derived(
		data.assignments.filter(assignment => {
			const searchValue = search.toLowerCase()
			return assignment.name.toLowerCase().includes(searchValue)
		}),
	)

	// Group filtered assignments by areaId
	type GroupedAssignments = Map<number, { areaName: string; assignments: Assignment[] }>
	let groupedAssignments = $derived.by(() => {
		const grouped = new Map<number, { areaName: string; assignments: Assignment[] }>()

		for (const assignment of filteredAssignments) {
			if (!assignment.areaId || !assignment.areaName) continue

			if (!grouped.has(assignment.areaId)) {
				grouped.set(assignment.areaId, {
					areaName: assignment.areaName,
					assignments: [],
				})
			}

			grouped.get(assignment.areaId)!.assignments.push(assignment)
		}

		return grouped
	})

	// Track which accordions are open (first area expanded by default)
	let accordionStates = $state<Record<number, boolean>>({})

	// Initialize accordion states from grouped assignments
	// This runs whenever groupedAssignments changes and ensures all area states are initialized
	$effect(() => {
		if (groupedAssignments.size > 0) {
			const areaIds = Array.from(groupedAssignments.keys())
			let hasInitialized = false

			for (const areaId of areaIds) {
				// Only initialize if not already set (preserve user's manual toggles)
				if (!(areaId in accordionStates)) {
					// First area is expanded by default, others are collapsed
					accordionStates[areaId] = !hasInitialized
					hasInitialized = true
				}
			}
		}
	})

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
			await invalidateAll()
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
					{#if groupedAssignments.size > 0}
						{#each Array.from(groupedAssignments.entries()) as [areaId, { areaName, assignments }]}
							<Accordion title={areaName} bind:isOpen={accordionStates[areaId]}>
								{#snippet children()}
									<div class="assignments-group">
										{#each assignments as assignment}
											<AssignmentCheckbox
												{assignment}
												checked={selectedTasks.has(assignment.id)}
												onchange={() => toggleTask(assignment.id)}
											/>
										{/each}
									</div>
								{/snippet}
							</Accordion>
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
		max-height: 450px;
		overflow-y: auto;
		margin-bottom: 2rem;
	}

	.assignments-group {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 2rem;
	}
</style>
