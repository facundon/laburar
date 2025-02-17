<script lang="ts">
	import Button from '$components/Button.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import LinkList from '$components/LinkList.svelte'

	const { data } = $props()
	const tasks = data.tasks
	let tasksWithoutArea = $derived(data.tasksWithoutArea)
</script>

{#snippet Actions()}
	<Button href={ROUTES.task.create} Icon={Plus}>Agregar Tarea</Button>
{/snippet}
<MainContainer title="Tareas" {Actions}>
	<div class="lists-container">
		<section>
			<LinkList entities={tasks} getHref={id => ROUTES.task.view(id)} />
		</section>
		{#if tasksWithoutArea.length > 0}
			<section>
				<h3>Tareas sin area asignada</h3>
				<LinkList entities={tasksWithoutArea} getHref={id => ROUTES.task.view(id)} />
			</section>
		{/if}
	</div>
</MainContainer>

<style>
	h3 {
		margin-top: 1rem;
		margin-bottom: 0.5rem;
		font-size: 1.25rem;
	}
	.lists-container {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 2rem;
	}
</style>
