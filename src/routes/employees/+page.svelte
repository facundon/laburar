<script lang="ts">
	import Button from '$components/Button.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'

	const { data } = $props()
	const employees = data.employees
</script>

<main class="container">
	<div class="header">
		<h1>Personal</h1>
		<Button href={ROUTES.employee.create}>
			<Plus style="margin-right: 5px;" />
			Agregar Personal
		</Button>
	</div>
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

	.header {
		margin-bottom: 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
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
