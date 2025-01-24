<script lang="ts">
	import Modal from '$components/Modal.svelte'
	import Rating from '$components/Rating.svelte'
	import Table from '$components/Table.svelte'
	import { AssignmentDifficulties, type Assignment } from '$models/assignment.svelte'
	import { SuggestedEmployee } from '$models/employee.svelte'
	import { suggestEmployeesForAssignment } from '$queries/assignments'
	import { Stars } from 'lucide-svelte'

	interface Props {
		assignments: Assignment[]
	}

	let { assignments }: Props = $props()
	let assignmentToSuggest = $state<Assignment | null>(null)
	let assignmentsWithoutEmployees = $derived(
		assignments.map(assignment => ({
			...assignment,
			areaName: assignment.areaName,
			taskName: assignment.taskName,
			difficulty: assignment.difficulty,
			frequency: assignment.frequency,
			suggest: () => (assignmentToSuggest = assignment),
		})),
	)

	let suggestions = $state<SuggestedEmployee[] | null>(null)

	function closeModal() {
		assignmentToSuggest = null
		suggestions = null
	}

	async function getSuggestionsForAssignment() {
		if (!assignmentToSuggest) return
		suggestions = await suggestEmployeesForAssignment(assignmentToSuggest.id, '2025-1-1', '2025-2-2')
	}

	async function handleConfirmSuggestion() {
		// implemnt
	}
</script>

<Table
	rows={assignmentsWithoutEmployees}
	columns={[
		{ field: 'areaName', headerName: 'Area' },
		{ field: 'taskName', headerName: 'Tarea' },
		{
			field: 'difficulty',
			headerName: 'Dificultad',
			formatValue: value => AssignmentDifficulties.find(d => d.value === value)?.label || String(value),
		},
		{ field: 'frequency', headerName: 'Frecuencia' },
		{
			field: 'suggest',
			headerName: 'Sugerir',
			width: 20,
			renderCell: onclick => ({
				component: Stars,
				props: { fill: 'var(--primary-main)', strokeWidth: 1, onclick, style: 'cursor: pointer;' },
			}),
		},
	]}
/>

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
			{#each suggestions as employee}
				<dl>
					<dt>Nombre</dt>
					<dd>{employee.name}</dd>
					<dt>Puntaje</dt>
					<dd>{employee.score}</dd>
					<dt>Eficiencia</dt>
					<dd><Rating rating={employee.efficiency} /></dd>
					<dt>Asignaciones actuales</dt>
					<dd>{employee.assignmentsDifficulties.length}</dd>
				</dl>
			{/each}
		{/if}
	</Modal>
{/if}
