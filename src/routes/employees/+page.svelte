<script lang="ts">
	import Button from '$components/Button.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import LinkList from '$components/LinkList.svelte'

	const { data } = $props()
	const employees = data.employees
</script>

{#snippet Actions()}
	<Button href={ROUTES.employee.create} Icon={Plus}>Agregar Personal</Button>
{/snippet}
<MainContainer title="Personal" {Actions}>
	<LinkList entities={employees} getHref={id => ROUTES.employee.view(id)} />
</MainContainer>
