<script lang="ts">
	import Button from '$components/Button.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import LinkList from '$components/LinkList.svelte'

	const { data } = $props()
	const areas = data.areas
	let areasWithoutTasks = $derived(data.areasWithoutTasks)
</script>

{#snippet Actions()}
	<Button href={ROUTES.area.create} Icon={Plus}>Agregar Area</Button>
{/snippet}
<MainContainer title="Areas" {Actions}>
	<div class="lists-container">
		<section>
			<LinkList entities={areas} getHref={id => ROUTES.area.view(id)} />
		</section>
		{#if areasWithoutTasks.length > 0}
			<section>
				<h3>Areas sin tareas asginadas</h3>
				<LinkList entities={areasWithoutTasks} getHref={id => ROUTES.area.view(id)} />
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
