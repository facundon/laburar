<script lang="ts">
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import Rating from '$components/Rating.svelte'
	import { SuggestedEmployee } from '$models/employee.svelte'
	import { EmployeeAssignment } from '$models/employeeAssignment.svelte'
	import { suggestEmployeesForAssignment } from '$queries/assignments'
	import { formatDate } from '$utils'
	import { Gauge, Stars } from 'lucide-svelte'

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

	let suggestions = $state<SuggestedEmployee[] | null>(null)
	let replacement = $state<SuggestedEmployee | null>(null)

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
		if (!replacement) return
		// implemnt
	}

	function getRealStartDate() {
		const today = new Date()
		if (!assignmentToSuggest || assignmentToSuggest.startDate <= today) return today
		return assignmentToSuggest.startDate
	}
</script>

{#if assignments.length > 0}
	<div class="wrapper">
		<h2>Tareas sin Personal Asignado</h2>
		{#each assignments as assignment}
			<div class="group">
				<p>{assignment.name}</p>
				<Button Icon={Stars} outlined onclick={() => (assignmentToSuggest = assignment)} style="height: fit-content;">Sugerir</Button>
			</div>
		{/each}
	</div>
{:else}
	{null}
{/if}

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
		{:else}
			{#if suggestions.length === 0}
				<p>No hay nadie disponible para cubrir la tarea 🥲</p>
			{:else}
				{@const daysToRepalce = Math.ceil(
					((assignmentToSuggest.endDate?.getTime() ?? 0) - (getRealStartDate().getTime() ?? 0)) / (1000 * 60 * 60 * 24),
				)}
				<p class="instruction">
					Selecciona quien va a ser el reemplazo desde el <span>{formatDate(assignmentToSuggest.startDate)}</span> al
					<span>{formatDate(assignmentToSuggest.endDate)}</span> <span> ({daysToRepalce} días)</span>
				</p>
			{/if}
			<div class="suggestions-wrapper">
				{#each suggestions as employee, index}
					{#snippet Suggestion()}
						<button class="suggestion {replacement === employee && 'selected'}" onclick={() => (replacement = employee)}>
							<div class="title">
								<img src={suggestionIcons.get(index)} alt="Puesto" />
								<div>
									<h4>{employee.name}</h4>
									<Rating rating={employee.efficiency} displayRating={false} />
									<div class="score">
										<Gauge size={16} color="var(--error-light)" />
										<span>{employee.score.toFixed(2)}</span>
									</div>
								</div>
							</div>
							<dl>
								<dt>Cantidad de tareas</dt>
								<dd>
									{#if employee.assignmentsDifficulties.length >= 3}<span>🔥</span>{/if}
									{employee.assignmentsDifficulties.length} tareas
								</dd>
								{#if employee?.startDate && employee.startDate < assignmentToSuggest!.endDate}
									{@const daysOut = Math.ceil(
										((employee.startDate?.getTime() ?? 0) - (assignmentToSuggest!.startDate?.getTime() ?? 0)) / (1000 * 60 * 60 * 24),
									)}
									<dt>Días disponibles</dt>
									<dd>
										{#if daysOut <= 3}<span>🤷‍♀️</span>{/if}
										{daysOut} días
									</dd>
									{#if employee.startDate}
										<dt>Próximas vacaciones</dt>
										<dd>{formatDate(employee.startDate)}</dd>
									{/if}
									{#if employee.endDate}
										<dt>Regresa el</dt>
										<dd>{formatDate(employee.endDate)}</dd>
									{/if}
								{/if}
							</dl>
						</button>
					{/snippet}
					<Suggestion />
				{/each}
			</div>
		{/if}
	</Modal>
{/if}

<style>
	.suggestions-wrapper {
		display: grid;
		grid-template-columns: repeat(auto-fit, 22rem);
		gap: 1rem;
		justify-content: center;
		max-width: 1280px;
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
		width: 22rem;
		height: calc(23rem / 1.618);
		position: relative;
	}

	.suggestion.selected {
		background-color: #5b3c69; /* Tomato color */
	}
	.suggestion.selected:hover {
		background-color: #5b3c69;
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
		object-fit: contain;
		height: 4.5rem;
		border-radius: 50%;
	}

	h4 {
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
	.group > p {
		font-weight: 500;
	}
	.group {
		display: flex;
		align-items: center;
		gap: 2rem;
		justify-content: space-between;
	}
</style>
