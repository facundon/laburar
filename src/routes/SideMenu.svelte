<script lang="ts">
	import { ROUTES } from '$routes'
	import { Home, Users, BriefcaseBusiness, Component, ClipboardX, TentTree } from 'lucide-svelte'
	import { page } from '$app/state'
	import { onMount } from 'svelte'

	let isSidebarOpen = $state(true)

	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen
	}

	onMount(() => {
		const mediaQuery = window.matchMedia('(max-width: 768px)')
		isSidebarOpen = !mediaQuery.matches
		mediaQuery.addEventListener('change', e => {
			isSidebarOpen = !e.matches
		})
	})

	function isActive(route: string) {
		const activeRoute = page.url.pathname
		if (route === '/') {
			return activeRoute === route
		}
		return activeRoute.startsWith(route)
	}
</script>

<div class="sidebar-wrapper">
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
			<li>
				<a href={ROUTES.absence.list} aria-label="Ausencias" class:active={isActive(ROUTES.absence.list)}>
					<ClipboardX />
					<span class="menu-text">Ausencias</span>
				</a>
			</li>
			<li>
				<a href={ROUTES.holiday.list} aria-label="Vacaciones" class:active={isActive(ROUTES.holiday.list)}>
					<TentTree />
					<span class="menu-text">Vacaciones</span>
				</a>
			</li>
		</ul>
	</nav>
	<button class="hamburger {isSidebarOpen ? 'open' : ''}" onclick={toggleSidebar} aria-label="Abrir menú">
		<span class="bar"></span>
		<span class="bar"></span>
		<span class="bar"></span>
	</button>
</div>

<style>
	.sidebar-wrapper {
		position: sticky;
		display: flex;
		gap: 1rem;
		height: 100vh;
		min-width: fit-content;
		top: 0;
		overflow: hidden;
	}
	.side-menu {
		min-width: 14vw;
		transition: min-width 0.3s ease;
		height: 100%;
		background-color: var(--gray-main);
		padding: 1rem;
		color: var(--gray-contrast);
	}

	.side-menu.closed {
		min-width: 60px;
	}

	.side-menu ul {
		list-style: none;
		padding: 0;
		margin-top: 2rem;
	}

	.side-menu li {
		margin-bottom: 0.5rem;
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

	.hamburger {
		background: none;
		border: none;
		cursor: pointer;
		z-index: 1000;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		margin-top: 1rem;
		width: 30px;
		height: 30px;
	}

	.bar {
		display: block;
		width: 25px;
		height: 2px;
		margin: 3px 0;
		background-color: var(--primary-contrast);
		transition: all 0.4s ease-in-out;
	}

	.hamburger.open .bar:nth-child(1) {
		transform: rotate(45deg) translate(6px, 5px);
	}

	.hamburger.open .bar:nth-child(2) {
		opacity: 0;
	}

	.hamburger.open .bar:nth-child(3) {
		transform: rotate(-45deg) translate(6px, -5px);
	}
</style>
