<script lang="ts">
	import FormGroup from '$components/FormGroup.svelte'
	import DateInput from '$components/DateInput.svelte'
	import type { Employee } from '$models/employee.svelte'

	interface Props {
		employee: Employee
		onsubmit: (event: Event) => void
		children: () => any
	}

	let { employee = $bindable(), onsubmit = $bindable(), children }: Props = $props()

	function handleDateChange(event: Event) {
		const startDateInput = event.target as HTMLInputElement
		employee.startDate = startDateInput.value ? new Date(startDateInput.value) : undefined
	}
</script>

<form {onsubmit}>
	<div class="group">
		<FormGroup label="Nombre" id="firstName">
			<input id="firstName" bind:value={employee.firstName} required />
		</FormGroup>
		<FormGroup label="Apellido" id="lastName">
			<input id="lastName" bind:value={employee.lastName} required />
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup label="Dirección" id="address">
			<input id="address" bind:value={employee.address} required />
		</FormGroup>
		<FormGroup label="Teléfono" id="phone">
			<input id="phone" bind:value={employee.phone} />
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup label="Fecha de inicio" id="startDate">
			<DateInput id="startDate" bind:value={employee.startDate} />
		</FormGroup>
	</div>
	{@render children()}
</form>

<style>
	.group {
		display: flex;
		gap: 3rem;
	}
</style>
