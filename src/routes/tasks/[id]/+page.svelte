<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { Delete, Pencil } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { goto } from '$app/navigation'

	const { data } = $props()
	const task = data.task

	let showModal = $state(false)

	async function deleteTask() {
		try {
			if (!task) return
			await invoke('delete_task_command', { id: task.id })
			goto(ROUTES.task.list, { invalidateAll: true })
		} catch (error) {
			console.error('Failed to delete task:', error)
		}
	}

	const openModal = () => (showModal = true)
	const closeModal = () => (showModal = false)

	function handleConfirm() {
		deleteTask()
		closeModal()
	}
</script>

{#if task}
	<MainContainer title={task.name}>
		{#if task.description}
			<strong>Descripción:</strong>
			<p class="description">{task.description}</p>
		{/if}

		<div class="actions">
			<Button outlined href={ROUTES.task.edit(task.id)} Icon={Pencil}>Editar</Button>
			<Button outlined variant="error" onclick={openModal} Icon={Delete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showModal}
			isDestructive
			message={`¿Estás seguro de que deseas eliminar la tarea "${task.name}"?`}
			onconfirm={handleConfirm}
			onclose={closeModal}
		/>
	</MainContainer>
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
