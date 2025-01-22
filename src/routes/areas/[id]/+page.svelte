<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { Delete, Pencil, ClipboardPlus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import { AssignmentDifficulties } from '$models/assignment.svelte.js'
	import { invalidateAll } from '$app/navigation'

	const { data } = $props()
	let area = $state(data.area)

	$effect(() => {
		if (data?.area !== area) area = data.area
	})

	let showDeleteAreaModal = $state(false)
	let assignmentToDelete = $state<{ name: string; id: number } | null>(null)
	let showDeleteAssignmentModal = $derived(assignmentToDelete !== null)

	const openDeleteAreaModal = () => (showDeleteAreaModal = true)
	const closeDeleteAreaModal = () => (showDeleteAreaModal = false)
	async function deleteArea() {
		try {
			if (!area) return
			await invoke('delete_area_command', { id: area.id })
			closeDeleteAreaModal()
			window.location.href = ROUTES.area.list
		} catch (error) {
			console.error('Failed to delete area:', error)
		}
	}

	const openDeleteAssignmentModal = (assignment: { name: string; id: number }) => (assignmentToDelete = assignment)
	const closeDeleteAssignmentModal = () => (assignmentToDelete = null)
	async function deleteAssignment() {
		try {
			if (!area || !assignmentToDelete) return
			await invoke('delete_assignment_command', { id: assignmentToDelete.id })
			closeDeleteAssignmentModal()
			await invalidateAll()
		} catch (error) {
			console.error('Failed to delete assignment:', error)
		}
	}

	const areaAssignmentsWithActions = $derived(
		area?.assignments.map(assignment => ({
			...assignment,
			name: assignment.name,
			delete: { name: assignment.taskName as string, id: assignment.id },
		})) || [],
	)
</script>

{#if area}
	{#snippet Actions()}
		{#if area}
			<Button variant="secondary" href={ROUTES.area.assignTasks(area.id)} Icon={ClipboardPlus}>Asignar Tareas</Button>
		{/if}
	{/snippet}
	<MainContainer title={area.name} {Actions}>
		{#if area.description}
			<strong>Descripción:</strong>
			<p class="description">{area.description}</p>
		{/if}

		<div class="actions">
			<Button outlined href={ROUTES.area.edit(area.id)} Icon={Pencil}>Editar</Button>
			<Button outlined variant="error" onclick={openDeleteAreaModal} Icon={Delete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showDeleteAreaModal}
			isDestructive
			message={`¿Estás seguro de que deseas eliminar la area "${area.name}"?`}
			onconfirm={deleteArea}
			onclose={closeDeleteAreaModal}
		/>
	</MainContainer>
	{#if area.assignments.length > 0}
		<MainContainer title="Tareas" style="margin-top: 1rem;">
			{#if area.assignments.length > 0}
				<Table
					rows={areaAssignmentsWithActions}
					columns={[
						{ field: 'taskName', headerName: 'Tarea' },
						{
							field: 'difficulty',
							headerName: 'Dificultad',
							formatValue: value => AssignmentDifficulties.find(d => d.value === value)?.label || value.toString(),
						},
						{ field: 'frequency', headerName: 'Frecuencia' },
						{
							field: 'delete',
							headerName: '',
							width: 20,
							renderCell: value => ({
								component: Delete,
								props: {
									onclick: () => openDeleteAssignmentModal(value),
									color: 'var(--error-main)',
									style: 'cursor: pointer;',
								},
							}),
						},
					]}
				/>
			{/if}
		</MainContainer>
		<Modal
			show={showDeleteAssignmentModal}
			isDestructive
			message={`¿Estás seguro de que deseas eliminar la tarea "${assignmentToDelete?.name}" del area ${area.name}?`}
			onconfirm={deleteAssignment}
			onclose={closeDeleteAssignmentModal}
		/>
	{/if}
{/if}

<style>
	p.description {
		margin-bottom: 0.5rem;
		color: #fff;
		white-space: pre-wrap;
	}
	.actions {
		margin-top: 3rem;
		display: flex;
		justify-content: space-between;
	}
</style>
