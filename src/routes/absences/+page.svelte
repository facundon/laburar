<script lang="ts">
	import Button from '$components/Button.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'

	const { data } = $props()
	const absences = data.absences
</script>

{#snippet Actions()}
	<Button href={ROUTES.absence.create} Icon={Plus}>Agregar Falta</Button>
{/snippet}

<MainContainer title="Faltas" {Actions}>
	{#if absences}
		{#each absences as absence}
			<div>{absence.absenceType}</div>
		{/each}
	{/if}
</MainContainer>
