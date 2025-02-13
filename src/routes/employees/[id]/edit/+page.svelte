<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import { Save } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import EmployeeForm from '$pages/employees/components/EmployeeForm.svelte'
	import { goto } from '$app/navigation'

	const { data } = $props()
	let employee = $state(data.employee)

	async function updateEmployee(e: Event) {
		e.preventDefault()
		if (!employee) return
		try {
			await invoke('update_employee_command', employee.toUpdateDTO())
			goto(ROUTES.employee.view(employee.id))
		} catch (error) {
			console.error('Failed to update employee:', error)
		}
	}
</script>

{#if employee}
	<MainContainer title={`Editar ${employee.name}`}>
		<EmployeeForm onsubmit={updateEmployee} bind:employee>
			<div class="actions">
				<Button variant="secondary" outlined href={ROUTES.employee.view(employee.id)}>Cancelar</Button>
				<Button Icon={Save} type="submit">Guardar</Button>
			</div>
		</EmployeeForm>
	</MainContainer>
{/if}

<style>
	.actions {
		margin-top: 1rem;
		display: flex;
		gap: 1rem;
	}
</style>
