<script lang="ts">
	import Button from '$components/Button.svelte'
	import { ROUTES } from '$routes'
	import { Eye, Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import { toYesNo } from '$utils'
	import { goto } from '$app/navigation'

	const { data } = $props()
	const absences = data.absences

	const absencesWithActions = $derived(
		absences?.map(absence => ({
			...absence,
			hours: absence.hours || 0,
			absenceDate: absence.absenceDate,
			isJustified: absence.isJustified,
			willReturn: absence.willReturn,
			isReturned: absence.isReturned,
			view: ROUTES.absence.view(absence.id),
		})),
	)
</script>

{#snippet Actions()}
	<Button href={ROUTES.absence.create} Icon={Plus}>Agregar Falta</Button>
{/snippet}

<MainContainer title="Ausencias" {Actions}>
	{#if absencesWithActions}
		<Table
			rows={absencesWithActions}
			columns={[
				{ field: 'absenceDate', headerName: 'Fecha', formatValue: value => value.toLocaleDateString() },
				{ field: 'hours', headerName: 'Horas' },
				{ field: 'employeeName', headerName: 'Personal' },
				{ field: 'isJustified', headerName: 'Justificada', formatValue: toYesNo },
				{ field: 'willReturn', headerName: 'Devolverá', formatValue: toYesNo },
				{ field: 'isReturned', headerName: 'Devolvió', formatValue: toYesNo },
				{
					field: 'view',
					width: 20,
					headerName: '',
					renderCell: href => ({
						component: Eye,
						props: { onclick: () => goto(href), color: 'var(--secondary-dark)', style: 'cursor: pointer;' },
					}),
				},
			]}
		/>
	{/if}
</MainContainer>
