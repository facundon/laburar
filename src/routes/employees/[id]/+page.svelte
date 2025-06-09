<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { differenceInYears } from 'date-fns'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { ClipboardPlus, Delete, Edit, Eye, Flame, Pencil, CalendarPlus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import Rating from '$components/Rating.svelte'
	import EditEmployeeAssignmentForm from '$pages/employees/[id]/EditEmployeeAssignmentForm.svelte'
	import type { EmployeeAssignment } from '$models/employeeAssignment.svelte.js'
	import { formatDateToFullDay, toYesNo } from '$utils'
	import { goto, invalidate, invalidateAll } from '$app/navigation'
	import { AssignmentColorMap } from '$models/assignment.svelte'

	const { data } = $props()
	let employee = $state(data.employee)
	let holidays = $state(data.holidays)
	let absencesWithActions = $derived(
		data.absences?.map(absence => ({
			...absence,
			absenceDate: absence.absenceDate,
			hours: absence.hours,
			employeeName: absence.employeeName,
			isJustified: absence.isJustified,
			willReturn: absence.willReturn,
			isReturned: absence.isReturned,
			view: ROUTES.absence.view(absence.id),
		})) || [],
	)

	let holidaysWithActions = $derived(
		holidays?.map(holiday => ({
			...holiday,
			startDate: holiday.startDate,
			endDate: holiday.endDate,
			daysOff: holiday.daysOff,
			notes: holiday.notes,
			view: ROUTES.holiday.view(holiday.id),
		})) || [],
	)

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
	let employeeAssignmentsWithActions = $derived(
		employee?.assignments.map(assignment => ({
			...assignment,
			name: assignment.name,
			difficulty: assignment.difficulty,
			efficiency: assignment.efficiency,
			isPrimary: assignment.isPrimary,
			areaName: assignment.areaName,
			assignedDate: assignment.assignedDate,
			taskName: assignment.taskName,
			delete: () => (assignmentToDelete = assignment),
			edit: () => (assignmentToEdit = assignment),
		})) || [],
	)

	let showAssignHolidaysConfirmation = $state(false)
	const openAssignHolidaysConfirmation = () => (showAssignHolidaysConfirmation = true)
	const closeAssignHolidaysConfirmation = () => (showAssignHolidaysConfirmation = false)
	async function assignHolidays() {
		if (!employee) return
		try {
			employee.accumulatedHolidays += employee.holidaysPerYear
			await invoke('update_employee_command', employee.toUpdateDTO())
			await invalidateAll()
			closeAssignHolidaysConfirmation()
		} catch (error) {
			employee.accumulatedHolidays -= employee.holidaysPerYear
			console.error('Failed to update employee:', error)
		}
	}

	let employeePrimaryAssignments = $derived(employeeAssignmentsWithActions.filter(assignment => assignment.isPrimary))
	let employeeSecondaryAssignments = $derived(employeeAssignmentsWithActions.filter(assignment => !assignment.isPrimary))
	// svelte-ignore state_referenced_locally
	employeePrimaryAssignments.sort((a, b) => {
		if (a.difficulty === b.difficulty) return b.efficiency - a.efficiency
		return a.difficulty - b.difficulty
	})
	// svelte-ignore state_referenced_locally
	employeeSecondaryAssignments.sort((a, b) => {
		if (a.difficulty === b.difficulty) return b.efficiency - a.efficiency
		return a.difficulty - b.difficulty
	})

	const getDifferenceInYears = (startDate: Date): number => differenceInYears(new Date(), startDate)
</script>

{#if employee}
	{#snippet Actions()}
		{#if employee}
			<div class="flex">
				<Button variant="secondary" onclick={openAssignHolidaysConfirmation} Icon={CalendarPlus}>Asignar Vacaciones</Button>
				<Button variant="secondary" href={ROUTES.employee.assignTasks(employee.id)} Icon={ClipboardPlus}>Asignar Tareas</Button>
			</div>
		{/if}
	{/snippet}
	<MainContainer title={employee.name} {Actions}>
		<div class="employee-info">
			<div class="column">
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
			</div>
			<div class="column">
				<p>
					<strong>Días de vacaciones por año:</strong>
					{employee.holidaysPerYear}
				</p>
				<p>
					<strong>Días de vacaciones acumulados:</strong>
					{employee.accumulatedHolidays}
				</p>
			</div>
		</div>
		<div class="actions">
			<Button outlined href={ROUTES.employee.edit(employee.id)} Icon={Pencil}>Editar</Button>
			<Button outlined variant="error" onclick={handleOpenDeleteEmployee} Icon={Delete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showDeleteEmployeeModal}
			isDestructive
			message={`¿Estás seguro de que deseas eliminar a "${employee.name}"?`}
			onconfirm={deleteEmployee}
			onclose={handleCloseDeleteEmployee}
		/>
	</MainContainer>

	{#if employeePrimaryAssignments.length > 0}
		<MainContainer title="Tareas Primarias Asignadas" style="margin-top: 1rem;">
			<Table
				rows={employeePrimaryAssignments}
				columns={[
					{
						field: 'difficulty',
						width: 20,
						headerName: '',
						renderCell: value => ({
							component: Flame,
							props: { color: 'var(--gray-light)', fill: AssignmentColorMap.get(value) },
						}),
					},
					{ field: 'name', headerName: 'Nombre' },
					{
						field: 'efficiency',
						headerName: 'Eficiencia',
						renderCell: value => ({ component: Rating, props: { rating: Number(value), displayRating: false } }),
					},
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
		</MainContainer>
	{/if}
	{#if employeeSecondaryAssignments.length > 0}
		<MainContainer title="Tareas de Remplazo" style="margin-top: 1rem;">
			<Table
				rows={employeeSecondaryAssignments}
				columns={[
					{
						field: 'difficulty',
						width: 20,
						headerName: '',
						renderCell: value => ({ component: Flame, props: { color: 'var(--gray-light)', fill: AssignmentColorMap.get(value) } }),
					},
					{ field: 'name', headerName: 'Nombre' },
					{
						field: 'efficiency',
						headerName: 'Eficiencia',
						renderCell: value => ({ component: Rating, props: { rating: Number(value), displayRating: false } }),
					},
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
		</MainContainer>
	{/if}
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
	{#if holidaysWithActions.length}
		<MainContainer title="Vacaciones Pasadas" style="margin-top: 1rem;">
			<Table
				rows={holidaysWithActions}
				columns={[
					{ field: 'startDate', headerName: 'Inicio', formatValue: value => formatDateToFullDay(value, true) },
					{ field: 'endDate', headerName: 'Fin', formatValue: value => formatDateToFullDay(value, true) },
					{ field: 'daysOff', headerName: 'Días' },
					{ field: 'notes', headerName: 'Notas' },
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
		message={`¿Estás seguro de que deseas eliminar Estás seguro de que deseas eliminar Estás seguro de que deseas eliminar la tarea asignada "${assignmentToDelete?.name}" de ${employee.name}?`}
		onconfirm={deleteAssignment}
		onclose={closeDeleteAssignmentModal}
	/>
	<Modal
		show={showAssignHolidaysConfirmation}
		message={`¿Estás seguro de que deseas asignar <strong>${employee.holidaysPerYear} días</strong> de vacaciones a ${employee.name}?`}
		onconfirm={assignHolidays}
		onclose={closeAssignHolidaysConfirmation}
		title="Asignar Vacaciones"
	/>
	{#if assignmentToEdit !== null}
		<EditEmployeeAssignmentForm onclose={closeEditAssignmentModal} bind:assignment={assignmentToEdit} {employee} />
	{/if}
{/if}

<style>
	p {
		margin-bottom: 0.5rem;
		color: #fff;
	}

	.flex {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.employee-info {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.column {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.actions {
		margin-top: 3rem;
		display: flex;
		justify-content: space-between;
	}
</style>
