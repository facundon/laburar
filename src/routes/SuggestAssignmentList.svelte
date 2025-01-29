<script lang="ts">
	import Button from '$components/Button.svelte'
	import SummaryContainer from '$components/SummaryContainer.svelte'
	import { EmployeeAssignment } from '$models/employeeAssignment.svelte'
	import SuggestAssignmentModal from '$pages/SuggestAssignmentModal.svelte'
	import { formatDateToFullDay } from '$utils'
	import { differenceInCalendarDays, max } from 'date-fns'
	import { Stars } from 'lucide-svelte'
	import Confetti from 'svelte-confetti'

	export type EmployeeAssignmentWithDates = EmployeeAssignment & { startDate: Date; endDate: Date }

	interface Props {
		assignments: EmployeeAssignmentWithDates[]
	}

	let { assignments }: Props = $props()
	let assignmentToSuggest = $state<EmployeeAssignmentWithDates | null>(null)

	function getAssignmentsMissingDays(assignment: EmployeeAssignmentWithDates | null) {
		if (!assignment) return 0
		const coveredDays = assignment.replacedDays
		return differenceInCalendarDays(new Date(assignment?.endDate), max([new Date(), assignment?.startDate])) - coveredDays
	}

	function closeModal() {
		assignmentToSuggest = null
	}

	function setAssignmentToSuggest(assignment: EmployeeAssignmentWithDates) {
		assignmentToSuggest = assignment
	}

	let currentAssignments = $derived(assignments.filter(a => a.startDate <= new Date() && getAssignmentsMissingDays(a) > 0))
	let nextAssignments = $derived(assignments.filter(a => a.startDate > new Date() && getAssignmentsMissingDays(a) > 0))
</script>

<SummaryContainer title={'Tareas sin Personal Asignado 🤹'}>
	{#if currentAssignments.length === 0 && nextAssignments.length === 0}
		<p class="empty">
			Todas las tareas estan asignadas 😎
			<Confetti x={[0.5, 3]} />
		</p>
	{/if}
	{#if currentAssignments.length > 0}
		<h3>Ahora</h3>
		{#each currentAssignments as assignment}
			<div class="group">
				<p>
					{assignment.name} (del <span class="date">{formatDateToFullDay(assignment.startDate, true)}</span> al
					<span class="date">{formatDateToFullDay(assignment.endDate, true)}</span>)
				</p>
				<Button Icon={Stars} outlined onclick={() => (assignmentToSuggest = assignment)} style="height: fit-content;">Sugerir</Button>
			</div>
		{/each}
	{/if}
	{#if nextAssignments.length > 0}
		<h3>Próximamente</h3>
		{#each nextAssignments as assignment}
			{#if getAssignmentsMissingDays(assignment) > 0}
				<div class="group">
					<p>
						{assignment.name} (del <span class="date">{formatDateToFullDay(assignment.startDate, true)}</span> al
						<span class="date">{formatDateToFullDay(assignment.endDate, true)}</span>)
					</p>
					<Button Icon={Stars} outlined onclick={() => setAssignmentToSuggest(assignment)} style="height: fit-content;">Sugerir</Button>
				</div>
			{/if}
		{/each}
	{/if}
</SummaryContainer>

{#if assignmentToSuggest}
	<SuggestAssignmentModal assignment={assignmentToSuggest} onclose={closeModal} />
{/if}

<style>
	span.date {
		color: var(--secondary-light);
	}

	.group > p {
		font-weight: 500;
		margin-block: 0;
	}
	.group {
		display: flex;
		align-items: center;
		gap: 2rem;
		justify-content: space-between;
		margin-bottom: 1rem;
	}
</style>
