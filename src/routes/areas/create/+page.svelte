<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import { Plus } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import { Area } from '$models/area.svelte'
	import AreaForm from '$pages/areas/components/AreaForm.svelte'
	import { goto } from '$app/navigation'

	let area = $state(new Area())

	async function createArea(e: Event) {
		e.preventDefault()
		try {
			await invoke('create_area_command', area.toCreateDTO())
			goto(ROUTES.area.list, { invalidateAll: true })
		} catch (error) {
			console.error('Failed to create area:', error)
		}
	}
</script>

<MainContainer title="Agregar Area">
	<AreaForm onsubmit={createArea} bind:area>
		<div class="actions">
			<Button outlined variant="secondary" href={ROUTES.area.list}>Cancelar</Button>
			<Button type="submit" Icon={Plus}>Crear</Button>
		</div>
	</AreaForm>
</MainContainer>

<style>
	.actions {
		display: flex;
		justify-content: space-between;
	}
</style>
