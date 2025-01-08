<script lang="ts">
	import Button from '$components/Button.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import LinkList from '$components/LinkList.svelte'

	const { data } = $props()
	const tasks = data.tasks
</script>

{#snippet Actions()}
	<Button href={ROUTES.task.create} Icon={Plus}>Agregar Tarea</Button>
{/snippet}
<MainContainer title="Tarea" {Actions}>
	<LinkList entities={tasks} getHref={id => ROUTES.task.view(id)} />
</MainContainer>
