<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import { Save } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import TaskForm from '$pages/tasks/components/TaskForm.svelte'
	import { goto } from '$app/navigation'

	const { data } = $props()
	let task = $state(data.task)

	async function updateTask(e: Event) {
		e.preventDefault()
		if (!task) return
		try {
			await invoke('update_task_command', task.toUpdateDTO())
			goto(ROUTES.task.view(task.id), { invalidateAll: true })
		} catch (error) {
			console.error('Failed to update task:', error)
		}
	}
</script>

{#if task}
	<MainContainer title={`Editar ${task.name}`}>
		<TaskForm onsubmit={updateTask} bind:task>
			<div class="actions">
				<Button variant="secondary" outlined href={ROUTES.task.view(task.id)}>Cancelar</Button>
				<Button variant="primary" Icon={Save} type="submit">Guardar</Button>
			</div>
		</TaskForm>
	</MainContainer>
{/if}

<style>
	.actions {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
	}
</style>
