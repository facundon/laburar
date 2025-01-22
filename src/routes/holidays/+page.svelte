<script lang="ts">
	import { goto } from '$app/navigation'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import { ROUTES } from '$routes'
	import { formatDate } from '$utils'
	import { Eye, Plus } from 'lucide-svelte'

	let { data } = $props()

	const holidaysWithActions = data.holidays.map(holiday => ({
		...holiday,
		daysOff: holiday.daysOff,
		startDate: holiday.startDate,
		endDate: holiday.endDate,
		notes: holiday.notes,
		employeeId: holiday.employeeId,
		view: () => goto(ROUTES.holiday.view(holiday.id)),
	}))
</script>

{#snippet Actions()}
	<Button href={ROUTES.holiday.create} Icon={Plus}>Agregar Vacaciones</Button>
{/snippet}
<MainContainer title="Vacaciones" {Actions}>
	<Table
		rows={holidaysWithActions}
		columns={[
			{ field: 'employeeName', headerName: 'Personal' },
			{ field: 'startDate', headerName: 'Inicio', formatValue: date => formatDate(date) },
			{ field: 'endDate', headerName: 'Fin', formatValue: date => formatDate(date) },
			{ field: 'notes', headerName: 'Notas', formatValue: notes => notes || '-', width: 300 },
			{
				field: 'view',
				headerName: '',
				renderCell: onclick => ({ component: Eye, props: { onclick, color: 'var(--secondary-dark)' } }),
				width: 20,
			},
		]}
	/>
</MainContainer>
