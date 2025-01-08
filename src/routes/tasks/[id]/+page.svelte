<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { Delete, Pencil } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'

	const { data } = $props()
	const task = data.task

	let showModal = $state(false)

	async function deleteTask() {
		try {
			if (!task) return
			await invoke('delete_task_command', { id: task.id })
			window.location.href = ROUTES.task.list
		} catch (error) {
			console.error('Failed to delete task:', error)
		}
	}

	function confirmDelete() {
		showModal = true
	}

	function handleClose() {
		showModal = false
	}

	function handleConfirm() {
		deleteTask()
		handleClose()
	}
</script>

<MainContainer title={task?.name || 'Unknown Task'}>
	{#if task}
		<p><strong>Descripción:</strong> {task.description}</p>
		<div class="actions">
			<Button href={ROUTES.task.edit(task.id)} Icon={Pencil}>Editar</Button>
			<Button style="margin-left: auto;" outlined variant="error" onclick={confirmDelete} Icon={Delete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showModal}
			title="Confirmar acción"
			isDestructive
			message={`¿Estás seguro de que deseas eliminar la tarea "${task.name}"?`}
			onconfirm={handleConfirm}
			onclose={handleClose}
		/>
	{:else}
		<p>Cargando...</p>
	{/if}
</MainContainer>

<style>
	p {
		margin-bottom: 0.5rem;
		color: #fff;
	}

	.actions {
		margin-top: 3rem;
		display: flex;
	}
</style>
