<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { Task } from '$models/task'
	import FormGroup from '$components/FormGroup.svelte'
	import TextArea from '$components/TextArea.svelte'

	let name = $state('')
	let description = $state('')
	let frequency = $state('')
	let difficulty = $state('')
	let area = $state('')

	async function createTask() {
		try {
			const task = new Task({
				id: 0,
				name,
				description,
				area,
				frequency,
				difficulty,
				createdAt: new Date(),
			})
			await invoke('create_task_command', task.toCreateDTO())
			window.location.href = ROUTES.task.list
		} catch (error) {
			console.error('Failed to create task:', error)
		}
	}
</script>

<MainContainer title="Agregar Tarea">
	<form onsubmit={createTask}>
		<FormGroup label="Nombre" id="name">
			<input id="name" bind:value={name} required />
		</FormGroup>
		<FormGroup label="Descripción" id="description">
			<TextArea id="description" bind:value={description}></TextArea>
		</FormGroup>
		<FormGroup label="Area" id="area">
			<input id="area" bind:value={area} />
		</FormGroup>
		<FormGroup label="Frecuencia" id="frequency">
			<input id="frequency" bind:value={frequency} required />
		</FormGroup>
		<FormGroup label="Dificultad" id="dificulty">
			<input id="dificulty" bind:value={difficulty} required />
		</FormGroup>
		<div class="actions">
			<Button outlined variant="secondary" href={ROUTES.task.list}>Cancelar</Button>
			<Button type="submit" Icon={Plus}>Crear</Button>
		</div>
	</form>
</MainContainer>

<style>
	.actions {
		display: flex;
		justify-content: space-between;
	}
</style>
