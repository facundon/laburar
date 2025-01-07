<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'

	const { data } = $props()
	const employee = data.employee

	async function deleteEmployee() {
		try {
			if (!employee) return
			await invoke('delete_employee_command', { id: employee.id })
			window.location.href = ROUTES.employee.list
		} catch (error) {
			console.error('Failed to delete employee:', error)
		}
	}
</script>

<main class="container">
	{#if employee}
		<h1>{employee.firstName} {employee.lastName}</h1>
		<p><strong>Teléfono:</strong> {employee.phone}</p>
		<p><strong>Dirección:</strong> {employee.address}</p>
		<p><strong>Fecha de inicio:</strong> {employee.startDate}</p>
		<div class="actions">
			<button onclick={deleteEmployee}>Eliminar</button>
			<a href={ROUTES.employee.edit(employee.id)} class="button">Editar</a>
		</div>
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

	p {
		margin-bottom: 0.5rem;
	}

	.actions {
		margin-top: 1rem;
		display: flex;
		gap: 1rem;
	}

	.button {
		display: inline-block;
		padding: 0.5rem 1rem;
		background-color: #007bff;
		color: #fff;
		text-decoration: none;
		border-radius: 4px;
	}

	.button:hover {
		background-color: #0056b3;
	}

	button {
		padding: 0.5rem 1rem;
		background-color: #dc3545;
		color: #fff;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}

	button:hover {
		background-color: #c82333;
	}
</style>
