<script lang="ts">
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { invoke } from '$invoke'
	import { Absence } from '$models/absence.svelte'
	import AbsenceForm from '$pages/absences/components/AbsenceForm.svelte'
	import { ROUTES } from '$routes'
	import { Plus } from 'lucide-svelte'

	let absence = $state(new Absence())

	const createAbsence = async () => {
		try {
			await invoke('create_absence_command', absence.toCreateDTO())
			window.location.href = ROUTES.absence.list
		} catch (error) {
			console.error('Failed to create absence:', error)
		}
	}
</script>

<MainContainer title="Nueva Falta">
	<AbsenceForm bind:absence onsubmit={createAbsence}>
		<div class="actions">
			<Button href={ROUTES.absence.list} outlined variant="secondary">Cancelar</Button>
			<Button type="submit" Icon={Plus}>Guardar</Button>
		</div>
	</AbsenceForm>
</MainContainer>

<style>
	.actions {
		display: flex;
		justify-content: space-between;
	}
</style>
