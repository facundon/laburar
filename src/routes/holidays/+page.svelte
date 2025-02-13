<script lang="ts">
	import { goto } from '$app/navigation'
	import { page } from '$app/state'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Select from '$components/Select.svelte'
	import Table from '$components/Table.svelte'
	import { ROUTES } from '$routes'
	import { formatDate } from '$utils'
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
				renderCell: onclick => ({ component: Eye, props: { onclick, color: 'var(--secondary-dark)', style: 'cursor: pointer;' } }),
				width: 20,
			},
		]}
	/>
</MainContainer>

<style>
	.picker {
		display: flex;
		gap: 1rem;
		align-items: center;
	}
</style>
