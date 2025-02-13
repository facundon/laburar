<script lang="ts">
	import { goto } from '$app/navigation'
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { invoke } from '$invoke'
	import { Holiday } from '$models/holiday.svelte'
	import HolidayForm from '$pages/holidays/HolidayForm.svelte'
	import { ROUTES } from '$routes'
	import { Save } from 'lucide-svelte'

	let holiday = $state(new Holiday())

	const createHoliday = async (e: Event) => {
		e.preventDefault()
		if (!holiday.employeeId) return
		try {
			await invoke('create_holiday_command', holiday.toCreateDTO())
			goto(ROUTES.holiday.list, { invalidateAll: true })
		} catch (error) {
			console.error('Failed to create holiday:', error)
		}
	}
</script>

<MainContainer title="Agregar Vacaciones">
	<HolidayForm bind:holiday onsubmit={createHoliday}>
		<div class="actions">
			<Button href={ROUTES.holiday.list} outlined variant="secondary">Cancelar</Button>
			<Button type="submit" Icon={Save} disabled={!holiday.employeeId}>Guardar</Button>
		</div>
	</HolidayForm>
</MainContainer>

<style>
	.actions {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
	}
</style>
