<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { Save } from 'lucide-svelte'

	let { data } = $props()
	const employee = data.employee

	async function assignTask() {
		if (!employee) return
		try {
			// await invoke('assign_task_command', task.toCreateDTO())
			window.location.href = ROUTES.employee.view(employee.id)
		} catch (error) {
			console.error('Failed to assign task:', error)
		}
	}
</script>

{#if employee}
	<MainContainer title={`Asignar Tarea a ${employee.name}`}>
		<form onsubmit={assignTask}>
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
</style>
