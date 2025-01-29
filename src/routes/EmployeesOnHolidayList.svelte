<script lang="ts">
	import SummaryContainer from '$components/SummaryContainer.svelte'
	import type { EmployeeOnHoliday } from '$models/employee.svelte'
	import { formatDateToFullDay } from '$utils'
	import Confetti from 'svelte-confetti'

	interface Props {
		employeesOnHoliday: EmployeeOnHoliday[]
	}

	let { employeesOnHoliday }: Props = $props()
	const currentlyOnHoliday = employeesOnHoliday.filter(employee => employee.currentlyOnHoliday)
	const upcomingHolidays = employeesOnHoliday.filter(employee => !employee.currentlyOnHoliday)
</script>

<SummaryContainer title="Personal de Vacaciones 🏖️">
	{#if employeesOnHoliday.length === 0}{/if}
	<p>
		Nadie de vacaciones 😮‍💨
		<Confetti x={[0.5, 3]} />
	</p>
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

<style>
	p {
		text-align: center;
		margin-block: 3rem;
	}
</style>
