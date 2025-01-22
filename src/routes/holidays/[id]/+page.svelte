<script lang="ts">
	import { invalidate } from '$app/navigation'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Modal from '$components/Modal.svelte'
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { formatDate } from '$utils'
	import { Delete, Pencil } from 'lucide-svelte'

	let { data } = $props()
	const holiday = data.holiday

	let showDeleteModal = $state(false)

	const openDeleteModal = () => (showDeleteModal = true)
	const closeDeleteModal = () => (showDeleteModal = false)

	async function deleteHoliday() {
		if (!holiday) return
		try {
			await invoke('delete_holiday_command', { id: holiday.id })
			invalidate(ROUTES.holiday.list)
			window.location.href = ROUTES.holiday.list
		} catch (err) {
			console.error('Failed to delete holiday:', err)
			closeDeleteModal()
		}
	}
</script>

{#if holiday}
	<MainContainer title="Vacaciones de {holiday.employeeName}">
		<div class="group">
			<dl>
				<dt>Personal</dt>
				<dd>{holiday.employeeName}</dd>
				<dt>Fecha de Inicio</dt>
				<dd>{formatDate(holiday.startDate)}</dd>
				<dt>Fecha de Fin</dt>
				<dd>{formatDate(holiday.endDate)}</dd>
				<!-- <dt>Dias Fuera</dt>
				<dd>{holiday.daysOff}</dd> -->
				<dt>Notas</dt>
				<dd>{holiday.notes || '-'}</dd>
			</dl>
		</div>
		<div class="actions">
			<Button Icon={Pencil} onclick={() => (window.location.href = ROUTES.holiday.edit(holiday.id))}>Editar</Button>
			<Button outlined variant="error" Icon={Delete} onclick={openDeleteModal}>Eliminar</Button>
		</div>
	</MainContainer>
	<Modal
		isDestructive
		onclose={closeDeleteModal}
		onconfirm={deleteHoliday}
		show={showDeleteModal}
		message="Esta seguro que desea eliminar las vacaciones del {formatDate(holiday.startDate)} al {formatDate(
			holiday.endDate,
		)} de {holiday.employeeName}?"
	/>
{/if}

<style>
	.group {
		display: flex;
		gap: 3rem;
	}

	dl {
		display: grid;
		grid-template-columns: 1fr 2fr;
		gap: 0.8rem;
	}

	dt {
		font-weight: bold;
	}

	.actions {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
	}
</style>
