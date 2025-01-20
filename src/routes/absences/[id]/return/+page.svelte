<script lang="ts">
	import { invalidate } from '$app/navigation'
	import Button from '$components/Button.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import NumberInput from '$components/NumberInput.svelte'
	import TextArea from '$components/TextArea.svelte'
	import { invoke } from '$invoke'
	import { AbsenceReturn } from '$models/absenceReturn.svelte.js'
	import { ROUTES } from '$routes'
	import { Save } from 'lucide-svelte'

	const { data } = $props()
	const absence = data.absence

	let absenceReturn = $state(new AbsenceReturn({ absenceId: absence?.id as number }))

	async function saveAbsenceReturn(event: Event) {
		event.preventDefault()
		try {
			if (!absence) return
			await invoke('create_absence_return_command', absenceReturn.toCreateDTO())
			invalidate(ROUTES.absence.list)
			window.location.href = ROUTES.absence.view(absence.id)
		} catch (err) {
			console.error('Failed to create absence return:', err)
		}
	}
</script>

<MainContainer title="Nueva Devolucion de Horas">
	<form onsubmit={saveAbsenceReturn}>
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
		<div class="group">
			{#if absence}
				<Button href={ROUTES.absence.view(absence.id)} outlined variant="secondary">Cancelar</Button>
				<Button type="submit" Icon={Save}>Guardar</Button>
			{/if}
		</div>
	</form>
</MainContainer>

<style>
	.group {
		display: flex;
		justify-content: space-between;
		gap: 3rem;
	}
</style>
