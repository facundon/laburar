<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { Task } from '$models/task.svelte'
	import TaskForm from '$pages/tasks/components/TaskForm.svelte'
	import { goto } from '$app/navigation'

	let task = $state(new Task())

	async function createTask(e: Event) {
		e.preventDefault()
		try {
			await invoke('create_task_command', task.toCreateDTO())
			goto(ROUTES.task.list)
		} catch (error) {
			console.error('Failed to create task:', error)
		}
	}
</script>

<MainContainer title="Agregar Tarea">
	<TaskForm onsubmit={createTask} bind:task>
		<div class="actions">
			<Button outlined variant="secondary" href={ROUTES.task.list}>Cancelar</Button>
			<Button type="submit" Icon={Plus}>Crear</Button>
		</div>
	</TaskForm>
</MainContainer>

<style>
	.actions {
		display: flex;
		justify-content: space-between;
	}
</style>
