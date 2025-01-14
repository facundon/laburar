<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { Delete, Pencil, ClipboardList } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import Rating from '$components/Rating.svelte'

	const { data } = $props()
	const area = data.area

	let showModal = $state(false)

	async function deleteArea() {
		try {
			if (!area) return
			await invoke('delete_area_command', { id: area.id })
			window.location.href = ROUTES.area.list
		} catch (error) {
			console.error('Failed to delete area:', error)
		}
	}

	const openModal = () => (showModal = true)
	const closeModal = () => (showModal = false)

	function handleConfirm() {
		deleteArea()
		closeModal()
	}
</script>

{#if area}
	{#snippet Actions()}
		<Button variant="secondary" href={ROUTES.area.assignTasks(area.id)} Icon={ClipboardList}>Asignar Tareas</Button>
	{/snippet}
	<MainContainer title={area.name} {Actions}>
		{#if area.description}
			<strong>Descripción:</strong>
			<p class="description">{area.description}</p>
		{/if}

		<div class="actions">
			<Button outlined href={ROUTES.area.edit(area.id)} Icon={Pencil}>Editar</Button>
			<Button outlined variant="error" onclick={openModal} Icon={Delete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showModal}
			title="Confirmar acción"
			isDestructive
			message={`¿Estás seguro de que deseas eliminar la area "${area.name}"?`}
			onconfirm={handleConfirm}
			onclose={closeModal}
		/>
	</MainContainer>
	{#if area.assignments.length > 0}
		<MainContainer title="Tareas" style="margin-top: 1rem;">
			{#if area.assignments.length > 0}
				<Table
					rows={area.assignments}
					columns={[
						{ field: 'taskName', headerName: 'Tarea' },
						{
							field: 'difficulty',
							headerName: 'Dificultad',
							renderCell: row => ({
								component: Rating,
								props: { rating: row.difficulty },
							}),
						},
						{ field: 'frequency', headerName: 'Frecuencia' },
					]}
				/>
			{/if}
		</MainContainer>
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
