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
		<p>Telefono: {employee.phone}</p>
		<p>Direccion: {employee.address}</p>
		<p>Fecha de inicio: {employee.startDate}</p>
		<button onclick={deleteEmployee}>Eliminar</button>
		<a href={ROUTES.employee.edit(employee.id)}>Editar</a>
	{:else}
		<p>Cargando...</p>
	{/if}
</main>

<style>
	/* ...existing code... */
</style>
