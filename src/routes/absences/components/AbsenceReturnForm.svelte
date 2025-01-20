<script lang="ts">
	import FormGroup from '$components/FormGroup.svelte'
	import NumberInput from '$components/NumberInput.svelte'
	import TextArea from '$components/TextArea.svelte'
	import type { AbsenceReturn } from '$models/absenceReturn'

	interface Props {
		absenceReturn: AbsenceReturn
		onsubmit: (event: Event) => void
		children?: () => any
	}

	let { absenceReturn = $bindable(), onsubmit = $bindable(), children }: Props = $props()
</script>

<form {onsubmit}>
	<div class="group">
		<FormGroup label="Fecha" id="returnDate">
			<input type="date" id="returnDate" required bind:value={absenceReturn.returnDate} />
		</FormGroup>
		<FormGroup label="Horas" id="returnedHours">
			<NumberInput id="returnedHours" required bind:value={absenceReturn.returnedHours} />
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup label="Notas" id="notes">
			<TextArea id="notes" bind:value={absenceReturn.notes}></TextArea>
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
