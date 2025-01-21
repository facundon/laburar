<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import { Save } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import AbsenceForm from '$pages/absences/components/AbsenceForm.svelte'

	const { data } = $props()
	let absence = $state(data.absence)

	async function updateAbsence(e: Event) {
		e.preventDefault()
		if (!absence) return
		try {
			await invoke('update_absence_command', absence.toUpdateDTO())
			window.location.href = ROUTES.absence.view(absence.id)
		} catch (error) {
			console.error('Failed to update absence:', error)
		}
	}
</script>

{#if absence}
	<MainContainer title={`Editar falta del ${absence.absenceDate.toLocaleDateString()} de ${absence.employeeName}`}>
		<AbsenceForm onsubmit={updateAbsence} bind:absence isEditMode>
			<div class="actions">
				<Button variant="secondary" outlined href={ROUTES.absence.view(absence.id)}>Cancelar</Button>
				<Button variant="primary" Icon={Save} type="submit">Guardar</Button>
			</div>
		</AbsenceForm>
	</MainContainer>
{/if}

<style>
	.actions {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
	}
</style>
