<script lang="ts">
	import { goto } from '$app/navigation'
	import { page } from '$app/state'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Select from '$components/Select.svelte'
	import Table from '$components/Table.svelte'
	import { ROUTES } from '$routes'
	import { formatDate, formatDateToFullDay } from '$utils'
	import { Eye, Plus } from 'lucide-svelte'

	let { data } = $props()
	const urlYear = new URLSearchParams(window.location.search).get('year')
	let year = $state(urlYear ? Number(urlYear) : new Date().getFullYear())

	const holidaysWithActions = $derived(
		data.holidays.map(holiday => ({
			...holiday,
			daysOff: holiday.daysOff,
			startDate: holiday.startDate,
			endDate: holiday.endDate,
			notes: holiday.notes,
			employeeId: holiday.employeeId,
			view: () => goto(ROUTES.holiday.view(holiday.id)),
		})),
	)

	const holidaysOnCourse = $derived(
		holidaysWithActions.filter(holiday => {
			const startDate = new Date(holiday.startDate)
			const endDate = new Date(holiday.endDate)
			const today = new Date()
			return startDate <= today && endDate >= today
		}),
	)

	const futureHolidays = $derived(
		holidaysWithActions.filter(holiday => {
			const startDate = new Date(holiday.startDate)
			const today = new Date()
			return startDate > today
		}),
	)

	const pastHolidays = $derived(
		holidaysWithActions.filter(holiday => {
			const endDate = new Date(holiday.endDate)
			const today = new Date()
			return endDate < today
		}),
	)

	const changeYear = async (year: number | null) => {
		if (!year || (urlYear && Number(urlYear) === year)) return
		page.url.searchParams.set('year', year.toString())
		await goto(page.url.href, { keepFocus: true, invalidateAll: true })
	}
</script>

{#snippet Actions()}
	<Button href={ROUTES.holiday.create} Icon={Plus}>Agregar Vacaciones</Button>
{/snippet}
<MainContainer title="Vacaciones" {Actions}>
	<div class="picker">
		<h3>Año</h3>
		<Select bind:value={year} onchange={changeYear} --width="100%">
			{#each Array.from({ length: 5 }, (_, i) => new Date().getFullYear() - i) as yearOpt}
				<option value={yearOpt}>{yearOpt}</option>
			{/each}
		</Select>
	</div>

	{#if holidaysWithActions.length === 0}
		<p class="empty-message">No hay vacaciones registradas 🤷‍♀️</p>
	{/if}

	{#if holidaysOnCourse.length > 0}
		<h3>En Curso</h3>
		<Table
			rows={holidaysOnCourse}
			columns={[
				{ field: 'employeeName', headerName: 'Personal' },
				{ field: 'startDate', headerName: 'Inicio', formatValue: date => formatDateToFullDay(date, true) },
				{ field: 'endDate', headerName: 'Fin', formatValue: date => formatDateToFullDay(date, true) },
				{ field: 'notes', headerName: 'Notas', formatValue: notes => notes || '-', width: 300 },
				{
					field: 'view',
					headerName: '',
					renderCell: onclick => ({ component: Eye, props: { onclick, color: 'var(--secondary-dark)', style: 'cursor: pointer;' } }),
					width: 20,
				},
			]}
		/>
	{/if}

	{#if futureHolidays.length > 0}
		<h3>Próximamente</h3>
		<Table
			rows={futureHolidays}
			columns={[
				{ field: 'employeeName', headerName: 'Personal' },
				{ field: 'startDate', headerName: 'Inicio', formatValue: date => formatDateToFullDay(date, true) },
				{ field: 'endDate', headerName: 'Fin', formatValue: date => formatDateToFullDay(date, true) },
				{ field: 'notes', headerName: 'Notas', formatValue: notes => notes || '-', width: 300 },
				{
					field: 'view',
					headerName: '',
					renderCell: onclick => ({ component: Eye, props: { onclick, color: 'var(--secondary-dark)', style: 'cursor: pointer;' } }),
					width: 20,
				},
			]}
		/>
	{/if}

	{#if pastHolidays.length > 0}
		<h3>Pasadas</h3>
		<Table
			rows={pastHolidays}
			columns={[
				{ field: 'employeeName', headerName: 'Personal' },
				{ field: 'startDate', headerName: 'Inicio', formatValue: date => formatDateToFullDay(date, true) },
				{ field: 'endDate', headerName: 'Fin', formatValue: date => formatDateToFullDay(date, true) },
				{ field: 'notes', headerName: 'Notas', formatValue: notes => notes || '-', width: 300 },
				{
					field: 'view',
					headerName: '',
					renderCell: onclick => ({ component: Eye, props: { onclick, color: 'var(--secondary-dark)', style: 'cursor: pointer;' } }),
					width: 20,
				},
			]}
		/>
	{/if}
</MainContainer>

<style>
	.picker {
		display: flex;
		gap: 1rem;
		align-items: center;
	}

	.empty-message {
		text-align: center;
		margin-top: 2rem;
	}
</style>
