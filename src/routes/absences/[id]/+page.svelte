<script lang="ts">
	import { invalidateAll } from '$app/navigation'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Modal from '$components/Modal.svelte'
	import Table from '$components/Table.svelte'
	import { invoke } from '$invoke'
	import type { AbsenceReturn } from '$models/absenceReturn.svelte.js'
	import { ROUTES } from '$routes'
	import { toYesNo } from '$utils'
	import { ClipboardPlus, Delete, Pencil } from 'lucide-svelte'

	let { data } = $props()
	let absence = $state(data.absence)

	$effect(() => {
		if (absence?.returns === data.absence?.returns) return
		absence = data.absence
	})

	let returnToDelete = $state<AbsenceReturn | null>(null)
	let deleteReturnOpen = $derived(returnToDelete !== null)

	let absenceReturnsWithActions = $derived(
		absence?.returns.map(r => ({ ...r, returnedHours: r.returnedHours, notes: r.notes, delete: () => (returnToDelete = r) })),
	)

	const closeDeleteReturnModal = () => (returnToDelete = null)

	async function deleteReturn() {
		if (!returnToDelete) return
		try {
			await invoke('delete_absence_return_command', { id: returnToDelete.id })
			closeDeleteReturnModal()
			await invalidateAll()
		} catch (err) {
			console.error('Failed to delete absence return:', err)
		}
	}
</script>

{#if absence}
	{#snippet Actions()}
		{#if absence}
			<Button href={ROUTES.absence.return(absence.id)} Icon={ClipboardPlus} variant="secondary">Nueva Devolución</Button>
		{/if}
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

{#if absenceReturnsWithActions?.length}
	<MainContainer title="Devoluciones" style="margin-top: 2rem;">
		<Table
			rows={absenceReturnsWithActions}
			columns={[
				{ field: 'returnDate', headerName: 'Fecha', formatValue: value => value.toLocaleDateString() },
				{ field: 'returnedHours', headerName: 'Horas' },
				{ field: 'notes', headerName: 'Notas', width: 700, formatValue: value => value || '-' },
				{ field: 'delete', headerName: '', renderCell: onclick => ({ component: Delete, props: { onclick, color: 'var(--error-main)' } }) },
			]}
		/>
	</MainContainer>
	<Modal
		show={deleteReturnOpen}
		isDestructive
		title="Confirmar acción"
		message={`¿Estás seguro de que deseas eliminar la devolucion del día ${returnToDelete?.returnDate.toLocaleDateString()}?`}
		onconfirm={deleteReturn}
		onclose={closeDeleteReturnModal}
	></Modal>
{/if}

<style>
	.actions {
		margin-top: 2rem;
		display: flex;
		justify-content: space-between;
	}
</style>
