<script lang="ts">
	import { page } from '$app/state'
	import { ChevronRight } from 'lucide-svelte'
	import { breadcrumbData } from '$context'

	const routes: { [key: string]: string } = {
		employees: 'Personal',
		areas: 'Areas',
		tasks: 'Tareas',
		return: 'Devolución',
		holidays: 'Vacaciones',
		'company-holidays': 'Feriados',
		'assign-task': 'Asignar Tarea',
		create: 'Crear',
		edit: 'Editar',
		absences: 'Ausencias',
		view: 'Ver',
	}

	const path = $derived(page.url.pathname.split('/').filter(Boolean))

	const breadcrumb = $derived(
		path.map((segment, index) => {
			let name = routes[segment as keyof typeof routes] || segment
			if (!isNaN(Number(segment))) name = breadcrumbData.name || 'Detalle'
			return {
				name,
				url: '/' + path.slice(0, index + 1).join('/'),
			}
		}),
	)
</script>

<nav class="breadcrumb">
	{#each breadcrumb as { name, url }, index}
		{#if index < breadcrumb.length - 1}
			<a href={url}>{name}</a>
			<ChevronRight size={18} strokeWidth={1} style="margin-inline: 0.5rem;" />
		{:else}
			<span class="active">{name}</span>
		{/if}
	{/each}
</nav>

<style>
	.breadcrumb {
		font-size: 1.1rem;
		position: sticky;
		top: 0;
		display: flex;
		align-items: center;
		padding-block: 1.5rem;
		margin-left: 3rem;
		background-color: var(--background-main);
	}

	.breadcrumb a {
		text-decoration: none;
		color: var(--primary-dark);
	}

	.breadcrumb span.active {
		font-weight: 600;
		color: var(--secondary-dark);
	}
</style>
