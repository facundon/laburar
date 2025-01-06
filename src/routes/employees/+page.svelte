<script lang="ts">
	import { ROUTES } from '$routes'

	const { data } = $props()
	const employees = data.employees
</script>

<main class="container">
	<h1>Employees</h1>
	<a href="/employees/new">Add New Employee</a>
	<ul>
		{#each employees as employee}
			<li>
				<a href={ROUTES.employee.view(employee.id)}>{employee.firstName} {employee.lastName}</a>
			</li>
		{/each}
	</ul>
</main>

<style>
	/* ...existing code... */
</style>
