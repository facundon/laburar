<script lang="ts">
	import FormGroup from '$components/FormGroup.svelte'
	import DateInput from '$components/DateInput.svelte'
	import type { Employee } from '$models/employee.svelte'
	import { formatDate } from '$utils'

	interface Props {
		employee: Employee
		onsubmit: (event: Event) => void
		children: () => any
	}

	let { employee = $bindable(), onsubmit = $bindable(), children }: Props = $props()
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
			<DateInput id="startDate" bind:value={employee.startDate} max={formatDate(new Date())} />
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup label="Días de vacaciones por año" id="holidayPerYear">
			<input id="holidayPerYear" bind:value={employee.holidaysPerYear} />
		</FormGroup>
		<FormGroup label="Días acumulados" id="accumulatedHolidays">
			<input id="accumulatedHolidays" bind:value={employee.accumulatedHolidays} />
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
