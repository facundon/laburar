<script lang="ts">
	import { page } from '$app/stores'
	import { derived } from 'svelte/store'
	import { ROUTES } from '$routes'

	const breadcrumb = derived(page, $page => {
		const path = $page.url.pathname.split('/').filter(Boolean)
		const routes: { [key: string]: string } = {
			employees: 'Personal',
			tasks: 'Tareas',
			create: 'Crear',
			edit: 'Editar',
			view: 'Ver',
		}
		return path.map((segment, index) => ({
			name: routes[segment as keyof typeof routes] || segment,
			url: '/' + path.slice(0, index + 1).join('/'),
		}))
	})
</script>

<nav class="breadcrumb">
	{#each $breadcrumb as { name, url }, index}
		{#if index < $breadcrumb.length - 1}
			<a href={url}>{name}</a>
			<span class="separator">|</span>
		{:else}
			<span class="active">{name}</span>
		{/if}
	{/each}
</nav>

<style>
	.breadcrumb {
		display: flex;
		align-items: center;
		font-size: 1.1rem;
		margin-bottom: 1rem;
	}

	.breadcrumb a {
		text-decoration: none;
		color: var(--primary-dark);
	}

	.breadcrumb span.active {
		font-weight: 600;
		color: var(--secondary-dark);
	}

	.separator {
		margin: 0 0.5rem;
	}
</style>
