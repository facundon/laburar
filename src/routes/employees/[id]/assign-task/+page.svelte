<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Checkbox from '$components/Checkbox.svelte' // Import the new Checkbox component
	import { Save } from 'lucide-svelte'

	let { data } = $props()
	const employee = data.employee
	const tasks = data.tasks
	let selectedTasks = new Set()

	function toggleTask(taskId: number) {
		if (selectedTasks.has(taskId)) selectedTasks.delete(taskId)
		else selectedTasks.add(taskId)
	}

	async function assignTasks() {
		if (!employee) return
		try {
			const tasksToAssign = Array.from(selectedTasks)
			await invoke('assign_tasks_to_employee_command', { employee_id: employee.id, task_ids: tasksToAssign })
			window.location.href = ROUTES.employee.view(employee.id)
		} catch (error) {
			console.error('Failed to assign task:', error)
		}
	}
</script>

{#if employee}
	<MainContainer title={`Asignar Tareas a ${employee.name}`}>
		<form onsubmit={assignTasks}>
			<div class="task-list">
				{#each tasks as task}
					<Checkbox id={task.id.toString()} onchange={() => toggleTask(task.id)} label={task.name} />
				{/each}
			</div>
			<div class="actions">
				<Button outlined variant="secondary" href={ROUTES.employee.view(employee.id)}>Cancelar</Button>
				<Button type="submit" style="margin-left: auto;">
					<Save style="margin-right: 5px;" />
					Asignar
				</Button>
			</div>
		</form>
	</MainContainer>
{/if}

<style>
	.actions {
		display: flex;
	}

	.task-list {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 10px;
		max-height: 450px;
		overflow-y: auto;
		margin-bottom: 2rem;
	}
</style>
