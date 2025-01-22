<script lang="ts">
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import { ROUTES } from '$routes'
	import { formatDateToFullDay } from '$utils'
	import { Delete, Plus } from 'lucide-svelte'
	import { isAfter } from 'date-fns'
	import Modal from '$components/Modal.svelte'
	import { CompanyHoliday } from '$models/company_holiday.svelte'
	import { invoke } from '$invoke'
	import { invalidateAll } from '$app/navigation'

	let { data } = $props()

	let holidayToDelete = $state<CompanyHoliday | null>(null)
	const setHolidayToDelete = (holiday: CompanyHoliday) => (holidayToDelete = holiday)
	const closeModal = () => (holidayToDelete = null)

	async function deleteHoliday() {
		if (!holidayToDelete) return
		try {
			await invoke('delete_company_holiday_command', { id: holidayToDelete.id })
			closeModal()
			invalidateAll()
		} catch (error) {
			console.error('Failed to delete company holiday:', error)
		}
	}

	const futureHolidays = $derived(data.companyHolidays.filter(holiday => isAfter(holiday.date, new Date())))
	const companyHolidaysWithActions = $derived(
		futureHolidays.map(holiday => ({
			...holiday,
			date: holiday.date,
			description: holiday.description,
			delete: () => setHolidayToDelete(holiday),
		})),
	)
</script>

{#snippet Actions()}
	<Button href={ROUTES.companyHoliday.create} Icon={Plus}>Agregar Feriado</Button>
{/snippet}
<MainContainer title="Feriados" {Actions}>
	<Table
		rows={companyHolidaysWithActions}
		columns={[
			{ field: 'description', headerName: 'Descripción' },
			{ field: 'date', headerName: 'Fecha', formatValue: formatDateToFullDay, isBold: true },
			{
				field: 'delete',
				headerName: '',
				renderCell: onclick => ({ component: Delete, props: { onclick, color: 'var(--error-main)', style: 'cursor: pointer;' } }),
				width: 20,
			},
		]}
	/>
</MainContainer>
<Modal
	show={!!holidayToDelete}
	isDestructive
	message="Esta seguro de eliminar el feriado {holidayToDelete?.description} del {formatDateToFullDay(holidayToDelete?.date ?? new Date())}"
	onclose={closeModal}
	onconfirm={deleteHoliday}
/>
