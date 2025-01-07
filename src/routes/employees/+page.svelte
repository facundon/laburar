<script lang="ts">
	import { ROUTES } from '$routes'

	const { data } = $props()
	const employees = data.employees
</script>

<main class="container">
	<h1>Employees</h1>
	<a href={ROUTES.employee.create}>Agregar Personal</a>
	<ul>
		{#each employees as employee}
			<li>
				<a href={ROUTES.employee.view(employee.id)}>{employee.name}</a>
			</li>
		{/each}
	</ul>
</main>

<style>
	/* ...existing code... */
</style>
