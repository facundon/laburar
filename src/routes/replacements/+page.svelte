<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation'
	import { page } from '$app/state'
	import Button from '$components/Button.svelte'
	import EmployeePicker from '$components/EmployeePicker.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Modal from '$components/Modal.svelte'
	import Table from '$components/Table.svelte'
	import { invoke } from '$invoke'
	import { Replacement } from '$models/replacement.svelte.js'
	import { formatDate, formatDateToFullDay } from '$utils'
	import { Delete } from 'lucide-svelte'

	let { data } = $props()
	let replacements = $derived(data.replacements)

	let replacementToDelete = $state<Replacement | null>(null)

	let employeeToFilter = $state<string | null>(null)

	let repalcementsWithActions = $derived(
		replacements.map(rep => ({
			...rep,
			assignment: rep.assignment,
			assignmentId: rep.assignmentId,
			createdAt: rep.createdAt,
			id: rep.id,
			notes: rep.notes,
			originalEmployeeId: rep.originalEmployeeId,
			originalEmployeeName: rep.originalEmployeeName,
			replacementEmployeeId: rep.replacementEmployeeId,
			replacementEmployeeName: rep.replacementEmployeeName,
			replacementEndDate: rep.replacementEndDate,
			replacementStartDate: rep.replacementStartDate,
			delete: () => (replacementToDelete = rep),
		})),
	)

	const closeDeleteModal = () => (replacementToDelete = null)

	async function handleDeleteReplacement() {
		if (!replacementToDelete) return
		try {
			await invoke('delete_replacement_command', { id: replacementToDelete.id })
			await invalidateAll()
			closeDeleteModal()
		} catch (err) {
			console.error(`Error deleting replacement: ${err}`)
		}
	}

	function handleChangeEmployee(employeeId: string | null) {
		if (employeeId) page.url.searchParams.set('employee_id', employeeId)
		else {
			page.url.searchParams.delete('employee_id')
			employeeToFilter = null
		}
		goto(page.url.href, { invalidateAll: true, keepFocus: true })
	}
</script>

<MainContainer title="Reemplazos">
	<div class="picker">
		<h4>Personal Reemplazando</h4>
		<EmployeePicker bind:value={employeeToFilter} onchange={handleChangeEmployee} --width="100%" />
		<Button style="flex-shrink: 0" variant="secondary" outlined onclick={() => handleChangeEmployee(null)}>Limpiar Filtros</Button>
	</div>
	<Table
		rows={repalcementsWithActions}
		columns={[
			{ field: 'originalEmployeeName', headerName: 'Personal' },
			{ field: 'replacementEmployeeName', headerName: 'Reemplazo' },
			{ field: 'replacementStartDate', headerName: 'Inicio', formatValue: date => formatDate(date) },
			{ field: 'replacementEndDate', headerName: 'Fin', formatValue: date => formatDate(date) },
			{ field: 'assignment', headerName: 'Asignación' },
			{
				field: 'delete',
				headerName: '',
				width: 20,
				renderCell: onclick => ({ component: Delete, props: { onclick, style: 'cursor: pointer', color: 'var(--error-main)' } }),
			},
		]}
	/>
</MainContainer>

{#if replacementToDelete}
	<Modal
		show
		isDestructive
		message="¿Estás seguro de que quieres eliminar el reemplazo del <strong>{formatDateToFullDay(
			replacementToDelete?.replacementStartDate,
		)}</strong> al <strong>{formatDateToFullDay(replacementToDelete?.replacementEndDate)}</strong>
 de <strong>{replacementToDelete?.replacementEmployeeName}</strong> para <strong>{replacementToDelete?.originalEmployeeName}</strong>?"
		onclose={closeDeleteModal}
		onconfirm={handleDeleteReplacement}
		title={`Eliminar reemplazo de ${replacementToDelete?.replacementEmployeeName}`}
	>
		{#if replacementToDelete.notes}
			<h4>Notas:</h4>
			<p>{replacementToDelete.notes}</p>
		{/if}
	</Modal>
{/if}

<style>
	.picker {
		display: flex;
		gap: 1rem;
		align-items: center;
		margin-bottom: 1rem;
	}

	.picker h4 {
		margin: 0;
		flex-shrink: 0;
	}

	p {
		white-space: pre-wrap;
	}
</style>
