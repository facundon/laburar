<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import { Save } from 'lucide-svelte'
	import FromGroup from '$components/FormGroup.svelte'
	import MainContainer from '$components/MainContainer.svelte'

	const { data } = $props()
	let task = data.task

	async function updateTask(e: Event) {
		e.preventDefault()
		if (!task) return
		try {
			await invoke('update_task_command', task.toUpdateDTO())
			window.location.href = ROUTES.task.view(task.id)
		} catch (error) {
			console.error('Failed to update task:', error)
		}
	}
</script>

<MainContainer title={`Editar ${task?.name}`}>
	{#if task}
		<form onsubmit={updateTask}>
			<FromGroup label="Nombre" id="name">
				<input id="name" bind:value={task.name} required />
			</FromGroup>
			<FromGroup label="Descripción" id="description">
				<textarea id="description" bind:value={task.description}></textarea>
			</FromGroup>
			<div class="actions">
				<Button variant="secondary" outlined href={ROUTES.employee.view(task.id)}>Cancelar</Button>
				<Button style="margin-left: auto;" variant="primary">
					<Save style="margin-right: 10px;" size={18} />
					Guardar
				</Button>
			</div>
		</form>
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
		margin-top: 1rem;
		display: flex;
		gap: 1rem;
	}
</style>
