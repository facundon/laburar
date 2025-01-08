<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { Employee } from '$models/employee'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import { Plus } from 'lucide-svelte'

	let firstName = ''
	let lastName = ''
	let address = ''
	let phone = ''
	let startDate: Date | undefined = undefined

	async function createEmployee() {
		try {
			const employee = new Employee({
				id: 0,
				firstName,
				lastName,
				address,
				phone,
				startDate,
				createdAt: new Date(),
			})
			await invoke('create_employee_command', employee.toCreateDTO())
			window.location.href = ROUTES.employee.list
		} catch (error) {
			console.error('Failed to create employee:', error)
		}
	}

	function handleDateChange(event: Event) {
		const startDateInput = event.target as HTMLInputElement
		startDate = startDateInput.value ? new Date(startDateInput.value) : undefined
	}
</script>

<MainContainer title="Agregar Personal">
	<form onsubmit={createEmployee}>
		<FormGroup label="Nombre" id="firstName">
			<input id="firstName" bind:value={firstName} required />
		</FormGroup>
		<FormGroup label="Apellido" id="lastName">
			<input id="lastName" bind:value={lastName} required />
		</FormGroup>
		<FormGroup label="Dirección" id="address">
			<input id="address" bind:value={address} required />
		</FormGroup>
		<FormGroup label="Teléfono" id="phone">
			<input id="phone" bind:value={phone} />
		</FormGroup>
		<FormGroup label="Fecha de inicio" id="startDate">
			<input id="startDate" type="date" oninput={handleDateChange} />
		</FormGroup>
		<div class="actions">
			<Button outlined variant="secondary" href={ROUTES.employee.list}>Cancelar</Button>
			<Button type="submit" style="margin-left: auto;">
				<Plus style="margin-right: 5px;" />
				Crear
			</Button>
		</div>
	</form>
</MainContainer>

<style>
	.actions {
		display: flex;
	}
</style>
