<script lang="ts">
	import Checkbox from '$components/Checkbox.svelte'
	import EmployeePicker from '$components/EmployeePicker.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import NumberInput from '$components/NumberInput.svelte'
	import Select from '$components/Select.svelte'
	import TextArea from '$components/TextArea.svelte'
	import { Absence, AbsenceTypes } from '$models/absence.svelte'

	interface Props {
		absence: Absence
		onsubmit: (event: Event) => void
		children?: () => any
	}

	let { absence = $bindable(), onsubmit = $bindable(), children }: Props = $props()
</script>

<form {onsubmit}>
	<div class="group">
		<FormGroup label="Personal" id="employee">
			<EmployeePicker id="employee" bind:value={absence.employeeId} />
		</FormGroup>
		<FormGroup label="Motivo" id="absenceType">
			<Select id="absenceType" bind:value={absence.absenceType} required>
				{#each AbsenceTypes as { label, value }}
					<option {value}>{label}</option>
				{/each}
			</Select>
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup label="Fecha" id="absenceDate">
			<input type="date" id="absenceDate" required bind:value={absence.absenceDate} />
		</FormGroup>
		<FormGroup label="Horas" id="hours">
			<NumberInput id="hours" required bind:value={absence.hours} />
		</FormGroup>
	</div>
	<div class="group">
		<Checkbox
			label="Es justificada"
			bind:checked={absence.isJustified}
			onchange={() => {
				if (!absence.isJustified) absence.willReturn = false
			}}
		/>
		{#if absence.isJustified}
			<Checkbox label="Va a devolver" bind:checked={absence.willReturn} />
		{/if}
	</div>
	<div class="group">
		<FormGroup label="Descripción" id="description">
			<TextArea id="description" bind:value={absence.description}></TextArea>
		</FormGroup>
	</div>
	{@render children?.()}
</form>

<style>
	.group {
		display: flex;
		justify-content: space-between;
		gap: 3rem;
	}
</style>
