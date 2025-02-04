<script lang="ts">
	import Button from '$components/Button.svelte'
	import SummaryContainer from '$components/SummaryContainer.svelte'
	import type { EmployeeOnHoliday } from '$models/employee.svelte'
	import { ROUTES } from '$routes'
	import { isToday } from 'date-fns'
	import { ClipboardX } from 'lucide-svelte'
	import Confetti from 'svelte-confetti'

	interface Props {
		employeesFutureAbsences: EmployeeOnHoliday[]
	}

	let { employeesFutureAbsences }: Props = $props()

	const todayAbsences = employeesFutureAbsences.filter(employee => isToday(employee.startDate))
	const futureAbsences = employeesFutureAbsences.filter(employee => !isToday(employee.startDate))
</script>

{#snippet Action()}
	<Button variant="secondary" outlined href={ROUTES.holiday.list} Icon={ClipboardX}>Ver Ausencias</Button>
{/snippet}
<SummaryContainer title="Personal Ausente 😤" {Action}>
	{#if employeesFutureAbsences.length === 0}
		<p class="empty">
			Nadie va a faltar 😮‍💨
			<Confetti x={[0.5, 3]} />
		</p>
	{/if}
	{#if todayAbsences.length > 0}
		<h3>Ahora</h3>
		{#each todayAbsences as employee}
			<p>{employee.name}</p>
		{/each}
	{/if}
	{#if futureAbsences.length > 0}
		<h3>Próximamente</h3>
		{#each futureAbsences as employee}
			<p>{employee.name}</p>
		{/each}
	{/if}
</SummaryContainer>
