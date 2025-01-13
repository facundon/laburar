<script lang="ts">
	import Button from '$components/Button.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import LinkList from '$components/LinkList.svelte'

	const { data } = $props()
	const areas = data.areas
</script>

{#snippet Actions()}
	<Button href={ROUTES.area.create} Icon={Plus}>Agregar Area</Button>
{/snippet}
<MainContainer title="Area" {Actions}>
	<LinkList entities={areas} getHref={id => ROUTES.area.view(id)} />
</MainContainer>
