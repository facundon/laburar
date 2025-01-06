<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'

	let { data } = $props()
	const employee = data.employee

	async function updateEmployee(event: Event) {
		event.preventDefault()
		if (employee) {
			try {
				await invoke('update_employee_command', employee.toCreateDTO())
				window.location.href = ROUTES.employee.view(employee.id)
			} catch (error) {
				console.error('Failed to update employee:', error)
			}
		}
	}
</script>

<main class="container">
	{#if employee}
		<h1>Editar Personal</h1>
		<form onsubmit={updateEmployee}>
			<input placeholder="Nombre" bind:value={employee.firstName} />
			<input placeholder="Apellido" bind:value={employee.lastName} />
			<input placeholder="Telefono" bind:value={employee.phone} />
			<input placeholder="Direccion" bind:value={employee.address} />
			<button type="submit">Actualizar</button>
		</form>
	{:else}
		<p>Cargando...</p>
	{/if}
</main>

<style>
	/* ...existing code... */
</style>
