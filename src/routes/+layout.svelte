<script lang="ts">
	import { ROUTES } from '$routes'
	import { onMount } from 'svelte'
	import { Home, Users, Plus } from 'lucide-svelte'
	import { page } from '$app/state'
	import { cssVariables } from '$theme'

	let isSidebarOpen = true

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
</script>

<main class="layout" style={cssVariables}>
	<nav class="side-menu {isSidebarOpen ? 'open' : 'closed'}">
		<ul>
			<li><a href="/" aria-label="Inicio" class:active={page.url.pathname === '/'}><Home /><span class="menu-text">Inicio</span></a></li>
			<li>
				<a href={ROUTES.employee.list} aria-label="Empleados" class:active={page.url.pathname === ROUTES.employee.list}
					><Users /><span class="menu-text">Empleados</span></a
				>
			</li>
			<li>
				<a href={ROUTES.employee.create} aria-label="Agregar Empleado" class:active={page.url.pathname === ROUTES.employee.create}
					><Plus /><span class="menu-text">Agregar Empleado</span></a
				>
			</li>
		</ul>
	</nav>
	<section class="content">
		<button class="hamburger {isSidebarOpen ? 'open' : ''}" on:click={toggleSidebar} aria-label="Abrir menú">
			<span class="bar"></span>
			<span class="bar"></span>
			<span class="bar"></span>
		</button>
		<div class="container">
			<slot></slot>
		</div>
	</section>
</main>

<style>
	@font-face {
		font-family: 'Urbanist';
		src: url('/fonts/Urbanist-VariableFont_wght.ttf') format('truetype');
		font-weight: 100 900;
		font-style: normal;
	}

	@font-face {
		font-family: 'Urbanist';
		src: url('/fonts/Urbanist-Italic-VariableFont_wght.ttf') format('truetype');
		font-weight: 100 900;
		font-style: italic;
	}

	:global(body) {
		font-family: 'Urbanist', sans-serif;
		margin: 0;
		padding: 0;
		background-color: #f8f9fa;
		color: var(--primary-contrast);
	}

	.layout {
		display: flex;
		height: 100vh;
	}

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
		background-color: var(--secondary-main);
	}

	.side-menu .menu-text {
		margin-left: 1rem;
	}

	.side-menu.closed .menu-text {
		display: none;
	}

	.content {
		flex-grow: 1;
		padding: 2rem;
		background-color: var(--primary-light);
		position: relative;
	}

	.container {
		max-width: 1024px;
		margin: 0 auto;
		padding: 1rem;
	}

	.hamburger {
		position: absolute;
		top: 1rem;
		left: 1rem;
		background: none;
		border: none;
		cursor: pointer;
		z-index: 1000;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
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
