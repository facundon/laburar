<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { Employee } from '$models/employee'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { Plus } from 'lucide-svelte'
	import EmployeeForm from '$pages/employees/components/EmployeeForm.svelte'

	let employee = $state(new Employee())

	async function createEmployee() {
		try {
			await invoke('create_employee_command', employee.toCreateDTO())
			window.location.href = ROUTES.employee.list
		} catch (error) {
			console.error('Failed to create employee:', error)
		}
	}
</script>

<MainContainer title="Agregar Personal">
	<EmployeeForm onsubmit={createEmployee} bind:employee>
		<div class="actions">
			<Button outlined variant="secondary" href={ROUTES.employee.list}>Cancelar</Button>
			<Button type="submit" Icon={Plus}>Crear</Button>
		</div>
	</EmployeeForm>
</MainContainer>

<style>
	.actions {
		display: flex;
		justify-content: space-between;
	}
</style>
