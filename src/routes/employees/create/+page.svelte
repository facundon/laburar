<script lang="ts">
	import { Employee } from '$models/employee'
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'

	let firstName = ''
	let lastName = ''
	let phone = ''
	let address = ''
	let startDate = new Date()

	async function addEmployee(event: Event) {
		event.preventDefault()
		let newEmployee = new Employee({
			firstName,
			lastName,
			phone,
			address,
			startDate: new Date(),
			createdAt: new Date(),
			id: 0,
		})
		try {
			newEmployee = await invoke('create_employee_command', newEmployee.toCreateDTO(), Employee.fromDTO)
			window.location.href = ROUTES.employee.view(newEmployee.id)
		} catch (error) {
			console.error('Failed to create employee:', error)
		}
	}
</script>

<main class="container">
	<h1>Agregar Personal</h1>
	<form on:submit={addEmployee}>
		<input placeholder="Nombre" bind:value={firstName} />
		<input placeholder="Apellido" bind:value={lastName} />
		<input placeholder="Telefono" bind:value={phone} />
		<input placeholder="Direccion" bind:value={address} />
		<input placeholder="Fecha de inicio" bind:value={startDate} />
		<button type="submit">Agregar</button>
	</form>
</main>

<style>
	/* ...existing code... */
</style>
