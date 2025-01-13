<script lang="ts">
	import { ROUTES } from '$routes'
	import { Home, Users, BriefcaseBusiness, Component } from 'lucide-svelte'
	import { page } from '$app/state'

	export let isSidebarOpen: boolean

	function isActive(route: string) {
		const activeRoute = page.url.pathname
		if (route === '/') {
			return activeRoute === route
		}
		return activeRoute.startsWith(route)
	}
</script>

<nav class="side-menu {isSidebarOpen ? 'open' : 'closed'}">
	<ul>
		<li><a href="/" aria-label="Inicio" class:active={isActive('/')}><Home /><span class="menu-text">Inicio</span></a></li>
		<li>
			<a href={ROUTES.employee.list} aria-label="Personal" class:active={isActive(ROUTES.employee.list)}>
				<Users />
				<span class="menu-text">Personal</span>
			</a>
		</li>
		<li>
			<a href={ROUTES.task.list} aria-label="Tareas" class:active={isActive(ROUTES.task.list)}>
				<BriefcaseBusiness />
				<span class="menu-text">Tareas</span>
			</a>
		</li>
		<li>
			<a href={ROUTES.area.list} aria-label="Areas" class:active={isActive(ROUTES.area.list)}>
				<Component />
				<span class="menu-text">Areas</span>
			</a>
		</li>
	</ul>
</nav>

<style>
	.side-menu {
		width: 250px;
		background-color: var(--gray-main);
		padding: 1rem;
		color: var(--gray-contrast);
		transition: width 0.3s ease;
		overflow: hidden;
		position: relative;
	}

	.side-menu.closed {
		width: 60px;
	}

	.side-menu ul {
		list-style: none;
		padding: 0;
		margin-top: 2rem;
	}

	.side-menu li {
		margin-bottom: 1rem;
	}

	.side-menu a {
		text-decoration: none;
		color: var(--gray-contrast);
		display: flex;
		align-items: center;
		padding: 0.5rem 1rem;
		border-radius: 4px;
		transition: background-color 0.3s;
	}

	.side-menu a:hover {
		background-color: var(--gray-light);
	}

	.side-menu a.active {
		background-color: var(--primary-dark);
	}

	.side-menu .menu-text {
		margin-left: 1rem;
	}

	.side-menu.closed .menu-text {
		display: none;
	}
</style>
