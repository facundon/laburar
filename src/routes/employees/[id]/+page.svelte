<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { differenceInYears } from 'date-fns'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { ClipboardPlus, Delete, Edit, Eye, Pencil } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import Rating from '$components/Rating.svelte'
	import EditEmployeeAssignmentForm from '$pages/employees/[id]/EditEmployeeAssignmentForm.svelte'
	import type { EmployeeAssignment } from '$models/employeeAssignment.svelte.js'
	import { toYesNo } from '$utils'
	import { goto, invalidate, invalidateAll } from '$app/navigation'

	const { data } = $props()
	let employee = $state(data.employee)
	const absencesWithActions =
		data.absences?.map(absence => ({
			...absence,
			absenceDate: absence.absenceDate,
			hours: absence.hours,
			employeeName: absence.employeeName,
			isJustified: absence.isJustified,
			willReturn: absence.willReturn,
			isReturned: absence.isReturned,
			view: ROUTES.absence.view(absence.id),
		})) || []

	$effect(() => {
		if (data?.employee !== employee) employee = data.employee
	})

	let assignmentToDelete = $state<{ name: string; id: number } | null>(null)
	let assignmentToEdit = $state<EmployeeAssignment | null>(null)
	let showDeleteEmployeeModal = $state(false)
	let showDeleteAssignmentModal = $derived(assignmentToDelete !== null)

	const handleOpenDeleteEmployee = () => (showDeleteEmployeeModal = true)
	const handleCloseDeleteEmployee = () => (showDeleteEmployeeModal = false)
	async function deleteEmployee() {
		try {
			if (!employee) return
			await invoke('delete_employee_command', { id: employee.id })
			handleCloseDeleteEmployee()
			await invalidate(ROUTES.employee.list)
			window.location.href = ROUTES.employee.list
		} catch (error) {
			console.error('Failed to delete employee:', error)
		}
	}

	const closeDeleteAssignmentModal = () => (assignmentToDelete = null)
	async function deleteAssignment() {
		try {
			if (!employee || !assignmentToDelete) return
			await invoke('delete_employee_assignment_command', { employee_id: employee.id, assignment_id: assignmentToDelete.id })
			await invalidateAll()
			closeDeleteAssignmentModal()
		} catch (error) {
			console.error('Failed to delete assignment:', error)
		}
	}

	const closeEditAssignmentModal = () => (assignmentToEdit = null)
	const employeeAssignmentsWithActions = $derived(
		employee?.assignments.map(assignment => ({
			...assignment,
			name: assignment.name,
			efficiency: assignment.efficiency,
			isPrimary: assignment.isPrimary,
			areaName: assignment.areaName,
			assignedDate: assignment.assignedDate,
			taskName: assignment.taskName,
			delete: () => (assignmentToDelete = assignment),
			edit: () => (assignmentToEdit = assignment),
		})) || [],
	)

	const getDifferenceInYears = (startDate: Date): number => differenceInYears(new Date(), startDate)
</script>

{#if employee}
	{#snippet Actions()}
		{#if employee}
			<Button variant="secondary" href={ROUTES.employee.assignTasks(employee.id)} Icon={ClipboardPlus}>Asignar Tareas</Button>
		{/if}
	{/snippet}
	<MainContainer title={employee.name} {Actions}>
		<p><strong>Teléfono:</strong> {employee.phone}</p>
		<p><strong>Dirección:</strong> {employee.address}</p>
		{#if employee.startDate}
			<p>
				<strong>Fecha de inicio:</strong>
				{employee.startDate.toLocaleDateString()}
			</p>
			<p>
				<strong>Antigüedad:</strong>
				{getDifferenceInYears(employee.startDate)} años
			</p>
		{/if}
		<div class="actions">
			<Button outlined href={ROUTES.employee.edit(employee.id)} Icon={Pencil}>Editar</Button>
			<Button outlined variant="error" onclick={handleOpenDeleteEmployee} Icon={Delete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showDeleteEmployeeModal}
			isDestructive
			title="Confirmar acción"
			message={`¿Estás seguro de que deseas eliminar a "${employee.name}"?`}
			onconfirm={deleteEmployee}
			onclose={handleCloseDeleteEmployee}
		/>
	</MainContainer>

	{#if employee.assignments.length > 0}
		<MainContainer title="Tareas Asignadas" style="margin-top: 1rem;">
			{#if employee.assignments.length > 0}
				<Table
					rows={employeeAssignmentsWithActions}
					columns={[
						{ field: 'name', headerName: 'Nombre' },
						{
							field: 'efficiency',
							headerName: 'Eficiencia',
							renderCell: value => ({ component: Rating, props: { rating: Number(value) } }),
						},
						{ field: 'isPrimary', headerName: 'Es Primaria', formatValue: toYesNo },
						{
							field: 'edit',
							width: 20,
							headerName: '',
							renderCell: onclick => ({ component: Edit, props: { onclick, color: 'var(--secondary-dark)', style: 'cursor: pointer;' } }),
						},
						{
							field: 'delete',
							width: 20,
							headerName: '',
							renderCell: onclick => ({ component: Delete, props: { onclick, color: 'var(--error-main)', style: 'cursor: pointer;' } }),
						},
					]}
				/>
			{/if}
		</MainContainer>
		{#if absencesWithActions.length}
			<MainContainer title="Ausencias" style="margin-top: 1rem;">
				<Table
					rows={absencesWithActions}
					columns={[
						{ field: 'absenceDate', headerName: 'Fecha', formatValue: value => value.toLocaleDateString() },
						{ field: 'hours', headerName: 'Horas' },
						{ field: 'isJustified', headerName: 'Justificada', formatValue: toYesNo },
						{ field: 'willReturn', headerName: 'Devolverá', formatValue: toYesNo },
						{ field: 'isReturned', headerName: 'Devolvió', formatValue: toYesNo },
						{
							field: 'view',
							width: 20,
							headerName: '',
							renderCell: href => ({
								component: Eye,
								props: { onclick: () => goto(href), color: 'var(--secondary-dark)', style: 'cursor: pointer;' },
							}),
						},
					]}
				/>
			</MainContainer>
		{/if}

		<Modal
			show={showDeleteAssignmentModal}
			isDestructive
			title="Confirmar acción"
			message={`¿Estás seguro de que deseas eliminar Estás seguro de que deseas eliminar Estás seguro de que deseas eliminar la tarea asignada "${assignmentToDelete?.name}" de ${employee.name}?`}
			onconfirm={deleteAssignment}
			onclose={closeDeleteAssignmentModal}
		/>
		{#if assignmentToEdit !== null}
			<EditEmployeeAssignmentForm onclose={closeEditAssignmentModal} bind:assignment={assignmentToEdit} {employee} />
		{/if}
	{/if}
{/if}

<style>
	p {
		margin-bottom: 0.5rem;
		color: #fff;
	}

	.actions {
		margin-top: 3rem;
		display: flex;
		justify-content: space-between;
	}
</style>
