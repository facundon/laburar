<script lang="ts">
	import DateInput from '$components/DateInput.svelte'
	import EmployeePicker from '$components/EmployeePicker.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import TextArea from '$components/TextArea.svelte'
	import { Holiday } from '$models/holiday.svelte'
	import { formatDate } from '$utils'

	interface Props {
		holiday: Holiday
		isEditMode?: boolean
		onsubmit: (event: Event) => void
		children: () => any
	}

	let { holiday = $bindable(), onsubmit, children, isEditMode = false }: Props = $props()
</script>

<form {onsubmit}>
	<div class="group">
		<FormGroup id="employeeId" label="Personal">
			<EmployeePicker bind:value={holiday.employeeId} required disabled={isEditMode} />
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup id="startDate" label="Fecha de Inicio">
			<DateInput id="startDate" bind:value={holiday.startDate} required />
		</FormGroup>
		<FormGroup id="endDate" label="Fecha de Fin">
			<DateInput id="endDate" bind:value={holiday.endDate} required min={formatDate(holiday.startDate)} />
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup id="notes" label="Notas">
			<TextArea id="notes" bind:value={holiday.notes} />
		</FormGroup>
	</div>
	{@render children?.()}
</form>

<style>
	.group {
		display: flex;
		gap: 3rem;
	}
</style>
