<script lang="ts">
	import MainContainer from '$components/MainContainer.svelte'
	import type { EmployeeOnHoliday } from '$models/employee.svelte'
	import { formatDate, formatDateToFullDay } from '$utils'

	interface Props {
		employeesOnHoliday: EmployeeOnHoliday[]
	}

	let { employeesOnHoliday }: Props = $props()
	const currentlyOnHoliday = employeesOnHoliday.filter(employee => employee.currentlyOnHoliday)
	const upcomingHolidays = employeesOnHoliday.filter(employee => !employee.currentlyOnHoliday)
</script>

{#if currentlyOnHoliday.length > 0 || upcomingHolidays.length > 0}
	<div class="wrapper">
		<h2>Personal de Vacaciones 🏖️</h2>
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
	</div>
{:else}
	{null}
{/if}

<style>
	.wrapper {
		color: #fff;
	}

	h2 {
		margin-top: 0.5rem;
	}

	h3 {
		color: var(--primary-main);
	}

	p {
		font-weight: 500;
	}

	p > span {
		font-weight: 600;
		color: var(--secondary-light);
	}
</style>
