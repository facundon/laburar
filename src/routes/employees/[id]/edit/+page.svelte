<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { differenceInYears, format } from 'date-fns'

	const { data } = $props()
	let employee = data.employee

	async function updateEmployee(e: Event) {
		e.preventDefault()
		if (!employee) return
		try {
			await invoke('update_employee_command', employee.toUpdateDTO())
			window.location.href = ROUTES.employee.view(employee.id)
		} catch (error) {
			console.error('Failed to update employee:', error)
		}
	}

	function handleDateChange(event: Event) {
		if (!employee) return
		const input = event.target as HTMLInputElement
		employee.startDate = input.value ? new Date(input.value) : undefined
	}
</script>

<main class="container">
	{#if employee}
		<h1>Editar {employee.firstName} {employee.lastName}</h1>
		<form onsubmit={updateEmployee}>
			<div class="form-group">
				<label for="firstName">Nombre</label>
				<input id="firstName" bind:value={employee.firstName} required />
			</div>
			<div class="form-group">
				<label for="lastName">Apellido</label>
				<input id="lastName" bind:value={employee.lastName} required />
			</div>
			<div class="form-group">
				<label for="phone">Teléfono</label>
				<input id="phone" bind:value={employee.phone} />
			</div>
			<div class="form-group">
				<label for="address">Dirección</label>
				<input id="address" bind:value={employee.address} required />
			</div>
			<div class="form-group">
				<label for="startDate">Fecha de inicio</label>
				<input
					id="startDate"
					type="date"
					value={employee.startDate ? format(employee.startDate, 'yyyy-MM-dd') : ''}
					oninput={handleDateChange}
				/>
			</div>
			<button type="submit" class="button">Guardar</button>
		</form>
	{:else}
		<p>Cargando...</p>
	{/if}
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
