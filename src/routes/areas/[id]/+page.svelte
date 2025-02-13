<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { Delete, Pencil, ClipboardPlus, Edit } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import { Assignment, AssignmentDifficulties } from '$models/assignment.svelte.js'
	import { goto, invalidateAll } from '$app/navigation'
	import EditAssignmentForm from '$pages/areas/[id]/EditAssignmentForm.svelte'

	const { data } = $props()
	let area = $state(data.area)

	$effect(() => {
		if (data?.area !== area) area = data.area
	})

	let showDeleteAreaModal = $state(false)
	let assignmentToDelete = $state<Assignment | null>(null)

	const openDeleteAreaModal = () => (showDeleteAreaModal = true)
	const closeDeleteAreaModal = () => (showDeleteAreaModal = false)
	async function deleteArea() {
		try {
			if (!area) return
			await invoke('delete_area_command', { id: area.id })
			closeDeleteAreaModal()
			goto(ROUTES.area.list)
		} catch (error) {
			console.error('Failed to delete area:', error)
		}
	}

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

	let assignmentToEdit = $state<Assignment | null>(null)
	const closeEditAssignmentModal = () => (assignmentToEdit = null)

	const areaAssignmentsWithActions = $derived(
		area?.assignments.map(assignment => ({
			...assignment.object,
			delete: () => (assignmentToDelete = assignment),
			edit: () => (assignmentToEdit = assignment),
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
							formatValue: value => AssignmentDifficulties.find(d => d.value === value)?.label || String(value),
						},
						{ field: 'frequency', headerName: 'Frecuencia' },
						{
							field: 'edit',
							headerName: '',
							width: 20,
							renderCell: onclick => ({
								component: Edit,
								props: {
									onclick,
									color: 'var(--secondary-dark)',
									style: 'cursor: pointer;',
								},
							}),
						},
						{
							field: 'delete',
							headerName: '',
							width: 20,
							renderCell: onclick => ({
								component: Delete,
								props: {
									onclick,
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
			show={!!assignmentToDelete}
			isDestructive
			message={`¿Estás seguro de que deseas eliminar la tarea "${assignmentToDelete?.name}" del area ${area.name}?`}
			onconfirm={deleteAssignment}
			onclose={closeDeleteAssignmentModal}
		/>
		{#if assignmentToEdit}
			<EditAssignmentForm assignment={assignmentToEdit} onclose={closeEditAssignmentModal} />
		{/if}
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
