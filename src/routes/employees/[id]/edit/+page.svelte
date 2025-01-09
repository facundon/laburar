<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { format } from 'date-fns'
	import Button from '$components/Button.svelte'
	import { Save } from 'lucide-svelte'
	import FromGroup from '$components/FormGroup.svelte'
	import MainContainer from '$components/MainContainer.svelte'

	const { data } = $props()
	let employee = data.employee

	async function updateEmployee(e: Event) {
		e.preventDefault()
		if (!employee) return
		try {
			await invoke('update_employee_command', employee.toUpdateDTO())
			window.location.href = ROUTES.employee.view(employee.id)
		} catch (error) {
			console.error('Failed to update employee:', error)
		}
	}

	function handleDateChange(event: Event) {
		if (!employee) return
		const input = event.target as HTMLInputElement
		employee.startDate = input.value ? new Date(input.value) : undefined
	}
</script>

{#if employee}
	<MainContainer title={`Editar ${employee.name}`}>
		<form onsubmit={updateEmployee}>
			<FromGroup label="Nombre" id="firstName">
				<input id="firstName" bind:value={employee.firstName} required />
			</FromGroup>
			<FromGroup label="Apellido" id="lastName">
				<input id="lastName" bind:value={employee.lastName} required />
			</FromGroup>
			<FromGroup label="Teléfono" id="phone">
				<input id="phone" bind:value={employee.phone} />
			</FromGroup>
			<FromGroup label="Dirección" id="address">
				<input id="address" bind:value={employee.address} required />
			</FromGroup>
			<FromGroup label="Fecha de inicio" id="startDate">
				<input
					id="startDate"
					type="date"
					value={employee.startDate ? format(employee.startDate, 'yyyy-MM-dd') : ''}
					oninput={handleDateChange}
				/>
			</FromGroup>
			<div class="actions">
				<Button variant="secondary" outlined href={ROUTES.employee.view(employee.id)}>Cancelar</Button>
				<Button style="margin-left: auto;" variant="primary">
					<Save style="margin-right: 10px;" size={18} />
					Guardar
				</Button>
			</div>
		</form>
	</MainContainer>
{/if}

<style>
	.actions {
		margin-top: 1rem;
		display: flex;
		gap: 1rem;
	}
</style>
