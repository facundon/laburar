<script lang="ts">
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import { ROUTES } from '$routes'
	import { toYesNo } from '$utils'
	import { ClipboardPlus, Delete, Pencil } from 'lucide-svelte'

	let { data } = $props()
	const absence = data.absence
</script>

{#if absence}
	{#snippet Actions()}
		<Button href={ROUTES.absence.return(absence.id)} Icon={ClipboardPlus} variant="secondary">Nueva Devolución</Button>
	{/snippet}
	<MainContainer title="Falta {absence.absenceDate.toLocaleDateString()} - {absence.employeeName}" {Actions}>
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
		<div class="actions">
			<Button Icon={Pencil} href={ROUTES.absence.edit(absence.id)} outlined>Editar</Button>
			<Button Icon={Delete} outlined variant="error">Eliminar</Button>
		</div>
	</MainContainer>
{/if}

{#if absence?.returns.length}
	<MainContainer title="Devoluciones" style="margin-top: 2rem;">
		<Table
			rows={absence.returns}
			columns={[
				{ field: 'returnDate', headerName: 'Fecha', formatValue: value => value.toLocaleDateString() },
				{ field: 'returnedHours', headerName: 'Horas' },
				{ field: 'notes', headerName: 'Notas', formatValue: value => value || '-' },
			]}
		/>
	</MainContainer>
{/if}

<style>
	.actions {
		margin-top: 2rem;
		display: flex;
		justify-content: space-between;
	}
</style>
