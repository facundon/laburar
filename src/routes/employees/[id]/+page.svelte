<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { differenceInYears, format } from 'date-fns'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { ClipboardList, Delete, Pencil } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'

	const { data } = $props()
	const employee = data.employee
	const assignments = data.assignments

	let showModal = $state(false)

	async function deleteEmployee() {
		try {
			if (!employee) return
			await invoke('delete_employee_command', { id: employee.id })
			window.location.href = ROUTES.employee.list
		} catch (error) {
			console.error('Failed to delete employee:', error)
		}
	}

	function confirmDelete() {
		showModal = true
	}

	function handleClose() {
		showModal = false
	}

	function handleConfirm() {
		deleteEmployee()
		handleClose()
	}

	function getDifferenceInYears(startDate: Date): number {
		return differenceInYears(new Date(), startDate)
	}
</script>

{#if employee}
	{#snippet Actions()}
		<Button variant="secondary" href={ROUTES.employee.assignTasks(employee.id)} Icon={ClipboardList}>Asignar Tareas</Button>
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
			<Button style="margin-left: auto;" outlined variant="error" onclick={confirmDelete} Icon={Delete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showModal}
			isDestructive
			title="Confirmar acción"
			message={`¿Estás seguro de que deseas eliminar a "${employee.name}"?`}
			onconfirm={handleConfirm}
			onclose={handleClose}
		/>
	</MainContainer>
	{#if assignments.length > 0}
		<MainContainer title="Tareas" style="margin-top: 2rem;">
			{#each assignments as assignment}
				<p>{assignment.difficulty}</p>
			{/each}
		</MainContainer>
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
	}
</style>
