<script lang="ts">
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'
</script>

{#snippet Actions()}
	<Button href={ROUTES.holiday.create} Icon={Plus}>Agregar Vacaciones</Button>
{/snippet}
<MainContainer title="Vacaciones" {Actions}>
	<Table rows={[]} columns={[]} />
</MainContainer>
