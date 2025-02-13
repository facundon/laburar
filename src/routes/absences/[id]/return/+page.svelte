<script lang="ts">
	import { goto, invalidate } from '$app/navigation'
	import Button from '$components/Button.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import NumberInput from '$components/NumberInput.svelte'
	import TextArea from '$components/TextArea.svelte'
	import { invoke } from '$invoke'
	import { AbsenceReturn } from '$models/absenceReturn.svelte.js'
	import AbsenceReturnForm from '$pages/absences/components/AbsenceReturnForm.svelte'
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
			goto(ROUTES.absence.view(absence.id))
		} catch (err) {
			console.error('Failed to create absence return:', err)
		}
	}
</script>

<MainContainer title="Nueva Devolucion de Horas">
	<AbsenceReturnForm onsubmit={saveAbsenceReturn} bind:absenceReturn>
		<div class="group">
			{#if absence}
				<Button href={ROUTES.absence.view(absence.id)} outlined variant="secondary">Cancelar</Button>
				<Button type="submit" Icon={Save}>Guardar</Button>
			{/if}
		</div>
	</AbsenceReturnForm>
</MainContainer>

<style>
	.group {
		display: flex;
		justify-content: space-between;
		gap: 3rem;
	}
</style>
