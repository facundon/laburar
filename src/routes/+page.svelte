<script lang="ts">
	import CongratsText from '$components/CongratsText.svelte'
	import EmployeesFutureAbsences from '$pages/EmployeesFutureAbsences.svelte'
	import EmployeesOnHolidayList from '$pages/EmployeesOnHolidayList.svelte'
	import ReplacementsSummary from '$pages/ReplacementsSummary.svelte'
	import SuggestAssignmentList from '$pages/SuggestAssignmentList.svelte'
	import Confetti from 'svelte-confetti'

	let { data } = $props()
	let employeesOnHoliday = $derived(data.employeesOnHoliday)
	let employeesFutureAbsences = $derived(data.employeesFutureAbsences)
	let replacements = $derived(data.replacements)

	let holidayAssignments = $derived(
		[...employeesOnHoliday, ...employeesFutureAbsences].flatMap(e =>
			e.assignments.flatMap(a => ({
				...a,
				replacements: a.replacements,
				areaName: a.areaName,
				taskName: a.taskName,
				efficiency: a.efficiency,
				areaId: a.areaId,
				employeeId: a.employeeId,
				isPrimary: a.isPrimary,
				taskId: a.taskId,
				assignmentId: a.assignmentId,
				id: a.id,
				startDate: e.startDate,
				endDate: e.endDate,
				name: a.name,
				assignedDate: a.assignedDate,
				createdAt: a.createdAt,
				toCreateDTO: a.toCreateDTO,
				toUpdateDTO: a.toUpdateDTO,
				replacedDays: a.replacedDays,
			})),
		),
	)
</script>

<main>
	<h1><CongratsText>Bienvenida !!!</CongratsText></h1>
	<div class="full-screen-confetti">
		<Confetti x={[-3, 5]} y={[0, 0.3]} delay={[100, 1000]} duration={3000} amount={500} fallDistance="50vh" />
	</div>
	<div class="grid-container">
		<div class="grid-item">
			<EmployeesOnHolidayList {employeesOnHoliday} />
		</div>
		<div class="grid-item">
			<EmployeesFutureAbsences {employeesFutureAbsences} />
		</div>
		<div class="grid-item">
			<SuggestAssignmentList assignments={holidayAssignments} />
		</div>
		<div class="grid-item">
			<ReplacementsSummary {replacements} />
		</div>
	</div>
</main>

<style>
	h1 {
		text-align: center;
		font-size: 5rem;
		margin-top: 2rem;
	}

	main {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.full-screen-confetti {
		position: fixed;
		top: -50px;
		left: 0;
		height: 100vh;
		width: 100vw;
		display: flex;
		justify-content: center;
		overflow: hidden;
		pointer-events: none;
	}

	.grid-container {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(600px, 1fr));
		gap: 1rem;
		width: 100%;
		padding: 2rem;
		box-sizing: border-box;
	}

	.grid-item {
		background-color: var(--gray-light);
		padding: 2rem;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	@media (max-width: 600px) {
		.grid-container {
			grid-template-columns: 1fr;
		}
	}
</style>
