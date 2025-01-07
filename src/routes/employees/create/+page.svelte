<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { Employee } from '$models/employee'

	let firstName = ''
	let lastName = ''
	let address = ''
	let phone = ''
	let startDate = ''

	async function createEmployee() {
		try {
			const employee = new Employee({
				id: 0,
				firstName,
				lastName,
				address,
				phone,
				startDate: startDate ? new Date(startDate) : undefined,
				createdAt: new Date(),
			})
			await invoke('create_employee_command', employee.toCreateDTO())
			window.location.href = ROUTES.employee.list
		} catch (error) {
			console.error('Failed to create employee:', error)
		}
	}
</script>

<main class="container">
	<h1>Agregar Personal</h1>
	<form on:submit|preventDefault={createEmployee}>
		<div class="form-group">
			<label for="firstName">Nombre</label>
			<input id="firstName" bind:value={firstName} required />
		</div>
		<div class="form-group">
			<label for="lastName">Apellido</label>
			<input id="lastName" bind:value={lastName} required />
		</div>
		<div class="form-group">
			<label for="address">Dirección</label>
			<input id="address" bind:value={address} required />
		</div>
		<div class="form-group">
			<label for="phone">Teléfono</label>
			<input id="phone" bind:value={phone} />
		</div>
		<div class="form-group">
			<label for="startDate">Fecha de inicio</label>
			<input id="startDate" type="date" bind:value={startDate} />
		</div>
		<button type="submit" class="button">Crear</button>
	</form>
</main>

<style>
	.container {
		max-width: 600px;
		margin: 0 auto;
		padding: 2rem;
		background-color: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	h1 {
		margin-bottom: 1rem;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		font-weight: bold;
	}

	input {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 4px;
	}

	.button {
		display: inline-block;
		padding: 0.5rem 1rem;
		background-color: #007bff;
		color: #fff;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}

	.button:hover {
		background-color: #0056b3;
	}
</style>
