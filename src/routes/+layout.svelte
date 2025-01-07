<script lang="ts">
	import { ROUTES } from '$routes'
</script>

<main class="layout">
	<nav class="side-menu">
		<ul>
			<li><a href="/">Home</a></li>
			<li><a href={ROUTES.employee.list}>Employees</a></li>
			<li><a href={ROUTES.employee.create}>Add Employee</a></li>
		</ul>
	</nav>
	<section class="content">
		<slot></slot>
	</section>
</main>

<style>
	.layout {
		display: flex;
	}

	.side-menu {
		width: 200px;
		background-color: #f6f6f6;
		padding: 1rem;
	}

	.side-menu ul {
		list-style: none;
		padding: 0;
	}

	.side-menu li {
		margin-bottom: 1rem;
	}

	.side-menu a {
		text-decoration: none;
		color: #0f0f0f;
	}

	.content {
		flex-grow: 1;
		padding: 1rem;
	}
</style>
