<script lang="ts">
	import MainContainer from '$components/MainContainer.svelte'
	import { toYesNo } from '$utils'

	let { data } = $props()
	const absence = data.absence
</script>

{#if absence}
	<MainContainer title="Falta {absence.absenceDate.toLocaleDateString()} - {absence.employeeName}">
		<p><strong>Horas:</strong> {absence.hours} hrs</p>
		<p><strong>Tipo:</strong> {absence.absenceType}</p>
		<p><strong>Descripción:</strong> {absence.description}</p>
		<p><strong>Justificada:</strong> {toYesNo(absence.isJustified)}</p>
		{#if absence.isJustified}
			<p><strong>Devolverá:</strong> {toYesNo(absence.willReturn)}</p>
			{#if absence.willReturn}
				<p><strong>Devolió: </strong> {toYesNo(absence.isReturned)}</p>
			{/if}
		{/if}
	</MainContainer>
{/if}
