<script lang="ts">
	import { onMount } from 'svelte'
	import { cssVariables } from '$theme'
	import Breadcrumb from '$components/Breadcrumb.svelte'
	import SideMenu from '$components/SideMenu.svelte'

	let { children } = $props()
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
</script>

<main class="layout" style={cssVariables}>
	<SideMenu {isSidebarOpen} />
	<section class="content">
		<button class="hamburger {isSidebarOpen ? 'open' : ''}" onclick={toggleSidebar} aria-label="Abrir menú">
			<span class="bar"></span>
			<span class="bar"></span>
			<span class="bar"></span>
		</button>
		<div class="container">
			<Breadcrumb />
			{@render children()}
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
		font-size: large;
		background-color: #f8f9fa;
		color: var(--primary-contrast);
	}
	:global(.lucide *) {
		vector-effect: non-scaling-stroke;
	}
	:global(input, textarea) {
		font-size: 1rem;
	}

	.layout {
		display: flex;
		height: 100vh;
	}

	.content {
		flex-grow: 1;
		padding: 2rem;
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
