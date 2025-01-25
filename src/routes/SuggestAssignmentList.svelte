<script lang="ts">
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import Rating from '$components/Rating.svelte'
	import { SuggestedEmployee } from '$models/employee.svelte'
	import type { EmployeeAssignment } from '$models/employeeAssignment.svelte'
	import { suggestEmployeesForAssignment } from '$queries/assignments'
	import { formatDate } from '$utils'
	import { Star, Stars } from 'lucide-svelte'

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
		// implemnt
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
			{/if}
			<div class="suggestions-wrapper">
				{#each suggestions as employee, index}
					{#snippet Suggestion()}
						<button class="suggestion">
							<div class="title">
								<img src={suggestionIcons.get(index)} alt="Puesto" />
								<div>
									<h4>{employee.name}</h4>
									<Rating rating={employee.efficiency} displayRating={false} />
								</div>
							</div>
							<dl>
								<dt>Cantidad de tareas</dt>
								<dd>{employee.assignmentsDifficulties.length} tareas</dd>
								{#if employee?.startDate && employee.startDate < assignmentToSuggest!.endDate}
									{@const daysOut = Math.ceil(
										((employee.startDate?.getTime() ?? 0) - (assignmentToSuggest!.startDate?.getTime() ?? 0)) / (1000 * 60 * 60 * 24),
									)}
									{#if employee.startDate}
										<dt>Próximas vacaciones</dt>
										<dd>{formatDate(employee.startDate)}</dd>
									{/if}
									{#if employee.endDate}
										<dt>Regresa el</dt>
										<dd>{formatDate(employee.endDate)}</dd>
									{/if}
									<dt>Días disponibles</dt>
									<dd>{daysOut} días</dd>
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
		grid-template-columns: repeat(auto-fit, 24rem);
		gap: 2rem;
		justify-content: center;
		min-width: 100%;
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
		width: 24rem;
		position: relative;
	}

	.suggestion:hover {
		background-color: var(--gray-light);
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

	dl {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.6rem;
		margin-top: 2rem;
		width: max-content;
	}

	dl > dt {
		font-weight: 500;
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
