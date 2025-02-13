<script lang="ts">
	import { goto } from '$app/navigation'
	import Button from '$components/Button.svelte'
	import DateInput from '$components/DateInput.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { invoke } from '$invoke'
	import { CompanyHoliday } from '$models/company_holiday.svelte'
	import { ROUTES } from '$routes'
	import { Save } from 'lucide-svelte'

	let companyHoliday = $state(new CompanyHoliday())

	const createCompanyHoliday = async (e: Event) => {
		e.preventDefault()
		try {
			await invoke('create_company_holiday_command', companyHoliday.toCreateDTO())
			goto(ROUTES.companyHoliday.list)
		} catch (error) {
			console.error('Failed to create company holiday:', error)
		}
	}
</script>

<MainContainer title="Agregar Feriado">
	<form onsubmit={createCompanyHoliday}>
		<div class="group">
			<FormGroup id="description" label="Descripción">
				<input id="description" bind:value={companyHoliday.description} required />
			</FormGroup>
			<FormGroup id="date" label="Fecha">
				<DateInput bind:value={companyHoliday.date} />
			</FormGroup>
		</div>

		<div class="actions">
			<Button href={ROUTES.companyHoliday.list} outlined variant="secondary">Cancelar</Button>
			<Button type="submit" Icon={Save}>Guardar</Button>
		</div>
	</form>
</MainContainer>

<style>
	.group {
		display: flex;
		gap: 3rem;
	}

	.actions {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
	}
</style>
