<script lang="ts">
	import Button from '$components/Button.svelte'
	import SummaryContainer from '$components/SummaryContainer.svelte'
	import type { EmployeeOnHoliday } from '$models/employee.svelte'
	import { ROUTES } from '$routes'
	import { formatDateToFullDay } from '$utils'
	import { TentTreeIcon } from 'lucide-svelte'
	import Confetti from 'svelte-confetti'

	interface Props {
		employeesOnHoliday: EmployeeOnHoliday[]
	}

	let { employeesOnHoliday }: Props = $props()
	const currentlyOnHoliday = employeesOnHoliday.filter(employee => employee.currentlyOnHoliday)
	const upcomingHolidays = employeesOnHoliday.filter(employee => !employee.currentlyOnHoliday)
</script>

{#snippet Action()}
	<Button variant="secondary" outlined href={ROUTES.holiday.list} Icon={TentTreeIcon}>Ver Vacaciones</Button>
{/snippet}

<SummaryContainer title="Personal de Vacaciones 🏖️" {Action}>
	{#if employeesOnHoliday.length === 0}
		<p class="empty">
			Nadie de vacaciones 😮‍💨
			<Confetti x={[0.5, 3]} />
		</p>
	{/if}
	{#if currentlyOnHoliday.length > 0}
		<h3>Ahora</h3>
		{#each currentlyOnHoliday as employee}
			<p>{employee.name} (regresa el <span>{formatDateToFullDay(employee.endDate)}</span>)</p>
		{/each}
	{/if}
	{#if upcomingHolidays.length > 0}
		<h3>Próximamente</h3>
		{#each upcomingHolidays as employee}
			<p>{employee.name} (sale el <span>{formatDateToFullDay(employee.startDate)}</span>)</p>
		{/each}
	{/if}
</SummaryContainer>
