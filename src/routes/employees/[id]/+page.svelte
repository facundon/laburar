<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { differenceInYears, format } from 'date-fns'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'

	const { data } = $props()
	const employee = data.employee

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

<main class="container">
	{#if employee}
		<h1>{employee.firstName} {employee.lastName}</h1>
		<p><strong>Teléfono:</strong> {employee.phone}</p>
		<p><strong>Dirección:</strong> {employee.address}</p>
		{#if employee.startDate}
			<p>
				<strong>Fecha de inicio:</strong>
				{format(employee.startDate, 'dd/MM/yyyy')}
				<span>(<strong>{getDifferenceInYears(employee.startDate)} años</strong> de antigüedad)</span>
			</p>
		{/if}
		<div class="actions">
			<Button href={ROUTES.employee.edit(employee.id)}>Editar</Button>
			<Button style="margin-left: auto;" outlined variant="error" onclick={confirmDelete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showModal}
			title="Confirm Deletion"
			message="Are you sure you want to delete this employee?"
			onconfirm={handleConfirm}
			onclose={handleClose}
		/>
	{:else}
		<p>Cargando...</p>
	{/if}
</main>

<style>
	.container {
		max-width: 600px;
		margin: 0 auto;
		padding: 2rem;
		background-color: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	h1 {
		margin-bottom: 1rem;
	}

	p {
		margin-bottom: 0.5rem;
	}

	.actions {
		margin-top: 1rem;
		display: flex;
		gap: 1rem;
	}
</style>
