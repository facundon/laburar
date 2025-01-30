<script lang="ts">
	import Button from '$components/Button.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { invoke } from '$invoke'
	import { Holiday } from '$models/holiday.svelte'
	import HolidayForm from '$pages/holidays/HolidayForm.svelte'
	import { ROUTES } from '$routes'
	import { Save } from 'lucide-svelte'

	let { data } = $props()
	let holiday = $state(new Holiday(data.holiday || {}))

	const editHoliday = async (e: Event) => {
		e.preventDefault()
		try {
			await invoke('update_holiday_command', holiday.toUpdateDTO())
			window.location.href = ROUTES.holiday.view(holiday.id)
		} catch (error) {
			console.error('Failed to update holiday:', error)
		}
	}
</script>

<MainContainer title="Editar Vacaciones de {holiday.employeeName}">
	<HolidayForm bind:holiday onsubmit={editHoliday} isEditMode>
		<div class="actions">
			<Button href={ROUTES.holiday.view(holiday.id)} outlined variant="secondary">Cancelar</Button>
			<Button type="submit" Icon={Save}>Guardar</Button>
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
