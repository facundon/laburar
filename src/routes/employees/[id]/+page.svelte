<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { differenceInYears, format } from 'date-fns'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { ClipboardPlus, Delete, Pencil } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import Table from '$components/Table.svelte'
	import Rating from '$components/Rating.svelte'

	const { data } = $props()
	const employee = data.employee

	let assignmentToDelete = $state<{ name: string; id: number } | null>(null)
	let showDeleteEmployeeModal = $state(false)
	let showDeleteAssignmentModal = $derived(assignmentToDelete !== null)

	const handleOpenDeleteEmployee = () => (showDeleteEmployeeModal = true)
	const handleCloseDeleteEmployee = () => (showDeleteEmployeeModal = false)
	async function deleteEmployee() {
		try {
			if (!employee) return
			await invoke('delete_employee_command', { id: employee.id })
			handleCloseDeleteEmployee()
			window.location.href = ROUTES.employee.list
		} catch (error) {
			console.error('Failed to delete employee:', error)
		}
	}

	const openDeleteAssignmentModal = (assignment: { name: string; id: number }) => (assignmentToDelete = assignment)
	const closeDeleteAssignmentModal = () => (assignmentToDelete = null)
	async function deleteAssignment() {
		try {
			if (!employee || !assignmentToDelete) return
			await invoke('delete_employee_assignment_command', { employee_id: employee.id, assignment_id: assignmentToDelete.id })
			// employee.assignments = employee.assignments.filter(assignment => assignment.id !== assignmentToDelete)
			closeDeleteAssignmentModal()
		} catch (error) {
			console.error('Failed to delete assignment:', error)
		}
	}

	const employeeAssignmentsWithActions =
		employee?.assignments.map(assignment => ({
			...assignment,
			name: assignment.name,
			actions: [
				{
					label: 'Editar',
				},
				{
					label: 'Eliminar',
					variant: 'error',
					onclick: () => openDeleteAssignmentModal(assignment),
				},
			],
		})) || []

	const getDifferenceInYears = (startDate: Date): number => differenceInYears(new Date(), startDate)
</script>

{#if employee}
	{#snippet Actions()}
		<Button variant="secondary" href={ROUTES.employee.assignTasks(employee.id)} Icon={ClipboardPlus}>Asignar Tareas</Button>
	{/snippet}
	<MainContainer title={employee.name} {Actions}>
		<p><strong>Teléfono:</strong> {employee.phone}</p>
		<p><strong>Dirección:</strong> {employee.address}</p>
		{#if employee.startDate}
			<p>
				<strong>Fecha de inicio:</strong>
				{format(employee.startDate, 'dd/MM/yyyy')}
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
						{ field: 'isPrimary', headerName: 'Es Primaria', formatValue: value => (value ? 'Sí' : 'No') },
						{
							field: 'actions',
							headerName: 'Acciones',
							renderCell: value => ({ component: Delete, props: { onclick: value[1].onclick, color: 'var(--error-main)' } }),
						},
					]}
				/>
			{/if}
		</MainContainer>
		<Modal
			show={showDeleteAssignmentModal}
			isDestructive
			title="Confirmar acción"
			message={`¿Estás seguro de que deseas eliminar Estás seguro de que deseas eliminar Estás seguro de que deseas eliminar la tarea asignada "${assignmentToDelete?.name}" de ${employee.name}?`}
			onconfirm={deleteAssignment}
			onclose={closeDeleteAssignmentModal}
		/>
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
