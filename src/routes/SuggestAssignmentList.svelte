<script lang="ts">
	import { invalidateAll } from '$app/navigation'
	import Button from '$components/Button.svelte'
	import DateInput from '$components/DateInput.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import Modal from '$components/Modal.svelte'
	import Rating from '$components/Rating.svelte'
	import { invoke } from '$invoke'
	import { SuggestedEmployee } from '$models/employee.svelte'
	import { EmployeeAssignment } from '$models/employeeAssignment.svelte'
	import { Replacement } from '$models/replacement.svelte'
	import { suggestEmployeesForAssignment } from '$queries/assignments'
	import { formatDate, formatDateToFullDay } from '$utils'
	import { differenceInCalendarDays, eachDayOfInterval, max, min } from 'date-fns'
	import { Gauge, Stars } from 'lucide-svelte'
	import Confetti from 'svelte-confetti'

	const suggestionIcons = new Map([
		[0, '/icon-first.png'],
		[1, '/icon-second.png'],
		[2, '/icon-third.png'],
	])

	interface Props {
		assignments: (EmployeeAssignment & { startDate: Date; endDate: Date })[]
	}

	let { assignments }: Props = $props()
	let assignmentToSuggest = $state<(EmployeeAssignment & { startDate: Date; endDate: Date }) | null>(null)

	function getAssignmentsMissingDays(assignment: (EmployeeAssignment & { startDate: Date; endDate: Date }) | null) {
		if (!assignment) return 0
		const coveredDays = assignment.replacedDays
		return differenceInCalendarDays(new Date(assignment?.endDate), max([new Date(), assignment?.startDate])) + 1 - coveredDays
	}

	let disabledDates = $derived(
		assignmentToSuggest?.replacements.flatMap(r =>
			eachDayOfInterval({ start: new Date(r.replacementStartDate), end: new Date(r.replacementEndDate) }).map(date => formatDate(date)),
		),
	)

	// If this is less than zero, the assignment is already covered
	let assignmentMissingDays = $derived(getAssignmentsMissingDays(assignmentToSuggest))

	function findFirstAvailableDateRange() {
		const startDate = max([assignmentToSuggest?.startDate || new Date(), new Date()])
		const endDate = new Date(assignmentToSuggest?.endDate || new Date())
		const interval = eachDayOfInterval({ start: startDate, end: endDate })

		for (let i = 0; i < interval.length; i++) {
			const start = interval[i]
			if (disabledDates?.includes(formatDate(start))) continue

			for (let j = i; j < interval.length; j++) {
				const end = interval[j]
				if (disabledDates?.includes(formatDate(end))) break

				return { start, end }
			}
		}
		return { startDate, endDate }
	}

	const { startDate, endDate } = findFirstAvailableDateRange()

	let suggestions = $state<SuggestedEmployee[] | null>(null)
	let replacement = $derived(
		new Replacement({
			assignmentId: assignmentToSuggest?.assignmentId,
			originalEmployeeId: assignmentToSuggest?.employeeId,
			replacementEndDate: endDate,
			replacementStartDate: startDate,
		}),
	)

	let error = $state<string | null>('')
	$effect(() => {
		if (
			disabledDates?.includes(formatDate(replacement.replacementStartDate)) ||
			disabledDates?.includes(formatDate(replacement.replacementEndDate))
		) {
			error = 'La fecha seleccionada ya está ocupada'
			return
		}
		error = null
	})

	function closeModal() {
		assignmentToSuggest = null
		suggestions = null
	}

	async function getSuggestionsForAssignment() {
		if (!assignmentToSuggest) return
		suggestions = await suggestEmployeesForAssignment(
			assignmentToSuggest.assignmentId,
			formatDate(assignmentToSuggest.startDate),
			formatDate(assignmentToSuggest.endDate),
		)
	}

	async function handleConfirmSuggestion() {
		if (!replacement.replacementEmployeeId || error) return
		try {
			await invoke('create_replacement_command', replacement.toCreateDTO())
			await invalidateAll()
			closeModal()
		} catch (err) {
			console.error('Error creating replacement', err)
		}
	}

	function handlePickSuggestedEmployee(employee: SuggestedEmployee) {
		if (!assignmentToSuggest) return
		replacement.replacementEmployeeId = employee.id
		if (employee?.startDate && employee.startDate < assignmentToSuggest.endDate) {
			replacement.replacementEndDate = min([assignmentToSuggest.endDate, employee.startDate])
		} else {
			replacement.replacementEndDate = assignmentToSuggest.endDate
		}
	}

	function findEmployeeReplacements(employeeId: number) {
		return assignmentToSuggest?.replacements
			.filter(a => a.replacementEmployeeId === employeeId)
			.sort((a, b) => a.replacementStartDate.getTime() - b.replacementStartDate.getTime())
	}

	let currentAssignments = $derived(assignments.filter(a => a.startDate <= new Date()))
	let nextAssignments = $derived(assignments.filter(a => a.startDate > new Date()))
</script>

<div class="wrapper">
	<h2>Tareas sin Personal Asignado</h2>
	{#if assignments.length === 0}
		<p>
			Todas las tareas estan asignadas 😎
			<Confetti x={[0.5, 3]} />
		</p>
	{/if}
	{#if currentAssignments.length > 0}
		<h3>Ahora</h3>
		{#each currentAssignments as assignment}
			{#if getAssignmentsMissingDays(assignment) > 0}
				<div class="group">
					<p>
						{assignment.name} (del <span class="date">{formatDateToFullDay(assignment.startDate, true)}</span> al
						<span class="date">{formatDateToFullDay(assignment.endDate, true)}</span>)
					</p>
					<Button Icon={Stars} outlined onclick={() => (assignmentToSuggest = assignment)} style="height: fit-content;">Sugerir</Button>
				</div>
			{/if}
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
					<Button Icon={Stars} outlined onclick={() => (assignmentToSuggest = assignment)} style="height: fit-content;">Sugerir</Button>
				</div>
			{/if}
		{/each}
	{/if}
</div>

{#if assignmentToSuggest}
	<Modal
		show={!!assignmentToSuggest}
		title="Sugerencias para {assignmentToSuggest.areaName} - {assignmentToSuggest.taskName}"
		onclose={closeModal}
		onconfirm={handleConfirmSuggestion}
		onmount={getSuggestionsForAssignment}
	>
		{#if suggestions === null}
			<p>Cargando...</p>
		{:else if suggestions.length === 0}
			<p>No hay nadie disponible para cubrir la tarea 🥲</p>
		{:else}
			<p class="instruction">
				A esta tarea le restan cubrir <span>{assignmentMissingDays} días</span>. Selecciona alguien y elige las fechas de inicio y fin.
			</p>
			<div class="suggestions-wrapper">
				{#each suggestions as employee, index}
					{@const employeeReplacements = findEmployeeReplacements(employee.id)}
					{#snippet Suggestion()}
						<button
							class="suggestion {replacement.replacementEmployeeId === employee.id && 'selected'}"
							onclick={() => handlePickSuggestedEmployee(employee)}
						>
							<div class="title">
								<img src={suggestionIcons.get(index)} alt="Puesto" />
								<div>
									<h4>{employee.name}</h4>
									<Rating rating={employee.efficiency} displayRating={false} />
									<div class="score">
										<Gauge size={16} color="var(--error-light)" />
										<span>{employee.score.toFixed(2)}</span>
									</div>
									{#if index === 0}<Confetti delay={[100, 100]} />{/if}
								</div>
							</div>
							<dl>
								<dt>Actualmente realiza</dt>
								<dd>
									{#if employee.assignmentsDifficulties.length >= 3}<span>🔥</span>{/if}
									{employee.assignmentsDifficulties.length} tareas
								</dd>
								{#if employee?.startDate && employee.startDate < assignmentToSuggest!.endDate}
									{@const daysOut = differenceInCalendarDays(employee.startDate, max([assignmentToSuggest!.startDate, new Date()])) + 1}
									<dt>Días disponibles</dt>
									<dd>
										{#if daysOut <= 3}<span>🤷‍♀️</span>{/if}
										{daysOut} días
									</dd>
								{/if}
							</dl>
							{#if employee.startDate}
								<div class="sub-group">
									<h4>Próximas vacaciones</h4>
									<p>
										Del <span class="date">{formatDateToFullDay(employee.startDate, true)}</span> al
										<span class="date">{formatDateToFullDay(employee.endDate!, true)}</span>
									</p>
								</div>
							{/if}
							{#if employeeReplacements?.length}
								<div class="sub-group">
									<h4>Reemplaza esta tarea</h4>
									{#each employeeReplacements as replacement}
										<p>
											{#if replacement.replacementStartDate.getTime() === replacement.replacementEndDate.getTime()}
												El día <span class="date">{formatDateToFullDay(replacement.replacementStartDate, true)}</span>
											{:else}
												Del <span class="date">{formatDateToFullDay(replacement.replacementStartDate, true)}</span> al
												<span class="date">{formatDateToFullDay(replacement.replacementEndDate, true)}</span>
											{/if}
										</p>
									{/each}
								</div>
							{/if}
						</button>
					{/snippet}
					<Suggestion />
				{/each}
			</div>
			<div class="date-pick">
				<FormGroup id="replacementStartDate" label="Fecha de inicio">
					<DateInput
						id="replacementStartDate"
						bind:value={replacement.replacementStartDate}
						max={formatDate(assignmentToSuggest.endDate)}
						min={formatDate(new Date())}
						{disabledDates}
						required
					/>
				</FormGroup>
				<FormGroup id="replacementEndDate" label="Fecha de fin">
					<DateInput
						id="replacementEndDate"
						bind:value={replacement.replacementEndDate}
						max={formatDate(assignmentToSuggest.endDate)}
						min={formatDate(replacement.replacementStartDate)}
						{disabledDates}
						required
					/>
				</FormGroup>
			</div>
		{/if}
		{#if error}
			<p style="color: var(--error-light);">{error}</p>
		{/if}
	</Modal>
{/if}

<style>
	span.date {
		color: var(--secondary-light);
	}
	.sub-group {
		margin-top: 0.5rem;
		width: 100%;
	}
	.sub-group > h4 {
		margin-block: 0;
		color: var(--primary-main);
	}

	.date-pick {
		margin-top: 2rem;
		width: 100%;
		display: flex;
		gap: 3rem;
		justify-content: center;
	}
	.suggestions-wrapper {
		display: grid;
		grid-template-columns: repeat(3, 20rem);
		gap: 1rem;
		justify-content: center;
	}

	@keyframes gradientBackground {
		0% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
		100% {
			background-position: 0% 50%;
		}
	}
	.suggestion {
		cursor: pointer;
		display: flex;
		padding: 1rem;
		color: #fff;
		font-size: inherit;
		flex-direction: column;
		align-items: center;
		border-radius: 1rem;
		box-shadow: 0 0 0.5rem rgba(0, 0, 0, 0.6);
		background-color: var(--gray-main);
		transition: background-color 0.2s;
		width: 20rem;
		position: relative;
	}

	.suggestion.selected {
		background: linear-gradient(270deg, #5b3c69, #333);
		background-size: 400% 400%;
		animation: gradientBackground 15s ease infinite;
		background-color: #5b3c69; /* Tomato color */
	}

	.suggestion:hover {
		background-color: var(--gray-light);
	}

	.score {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		align-items: center;
		gap: 0.2rem;
	}

	.score > span {
		font-size: 0.8rem;
		color: var(--error-light);
	}

	.suggestion .title {
		width: 100%;
		display: flex;
		align-items: end;
		gap: 0.5rem;
	}

	.suggestion:first-child::before {
		content: '';
		position: absolute;
		top: -5px;
		left: -5px;
		right: -5px;
		bottom: -5px;
		border-color: var(--primary-light);
		border-radius: 1rem;
		animation: glowing 2s infinite;
	}

	@keyframes glowing {
		0% {
			box-shadow: 0 0 0.5rem var(--primary-light);
		}
		50% {
			box-shadow: 0 0 3rem var(--primary-light);
		}
		100% {
			box-shadow: 0 0 0.5rem var(--primary-light);
		}
	}

	p.instruction {
		margin-top: 0;
		margin-bottom: 2rem;
	}

	p.instruction > span {
		font-weight: 600;
		color: var(--secondary-dark);
	}

	p.instruction > span:nth-child(3) {
		color: var(--primary-dark);
	}
	dl {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		margin-top: 1.2rem;
		width: 100%;
	}

	dl > dt {
		font-weight: 500;
		margin-top: 0.3rem;
		white-space: nowrap;
		text-align: start;
		color: var(--secondary-light);
	}

	dl > dd {
		text-align: end;
	}

	img {
		/* object-fit: contain; */
		height: 4.5rem;
		width: 3.8rem;
		border-radius: 50%;
	}

	.title h4 {
		text-align: start;
		margin: 0.5rem 0;
		margin-left: 0.3rem;
	}

	h2 {
		margin-top: 0.5rem;
	}
	.wrapper {
		color: #fff;
	}
	.wrapper > p {
		text-align: center;
		margin-block: 3rem;
	}

	.wrapper > h3 {
		color: var(--primary-main);
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
