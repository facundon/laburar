<script lang="ts">
	import { ROUTES } from '$routes'

	const { data } = $props()
	const employees = data.employees
</script>

<main class="container">
	<h1>Empleados</h1>
	<a href={ROUTES.employee.create} class="button">Agregar Personal</a>
	<ul>
		{#each employees as employee}
			<li>
				<a href={ROUTES.employee.view(employee.id)}>{employee.name}</a>
			</li>
		{/each}
	</ul>
</main>

<style>
	.container {
		max-width: 800px;
		margin: 0 auto;
		padding: 2rem;
		background-color: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	h1 {
		margin-bottom: 1rem;
	}

	.button {
		display: inline-block;
		padding: 0.5rem 1rem;
		background-color: #007bff;
		color: #fff;
		text-decoration: none;
		border-radius: 4px;
		margin-bottom: 1rem;
	}

	.button:hover {
		background-color: #0056b3;
	}

	ul {
		list-style: none;
		padding: 0;
	}

	li {
		margin-bottom: 0.5rem;
	}

	a {
		text-decoration: none;
		color: #007bff;
	}

	a:hover {
		text-decoration: underline;
	}
</style>
