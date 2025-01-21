<script lang="ts">
	import { cssVariables } from '$theme'
	import Breadcrumb from '$components/Breadcrumb.svelte'
	import SideMenu from '$components/SideMenu.svelte'

	let { children } = $props()
</script>

<main style={cssVariables}>
	<SideMenu />
	<section>
		<Breadcrumb />
		<div class="container">
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
		color: #333;
	}
	:global(.lucide *) {
		vector-effect: non-scaling-stroke;
	}
	:global(input, textarea, select) {
		font-family: 'Urbanist', sans-serif;
		font-size: 1rem;
	}

	:global(input[type='number'])::-webkit-outer-spin-button,
	:global(input[type='number'])::-webkit-inner-spin-button {
		appearance: none;
		margin: 0;
	}

	:global(select) {
		appearance: none;
		padding: 0.5rem;
	}

	main {
		display: flex;
	}

	section {
		flex-grow: 1;
		flex-shrink: 0;
	}

	.container {
		max-width: 1920px;
		margin: 0 auto;
		padding-inline: 5rem;
		padding-bottom: 2rem;
		padding-top: 0.5rem;
	}
</style>
