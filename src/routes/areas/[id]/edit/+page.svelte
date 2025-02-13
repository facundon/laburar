<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import { Save } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'
	import AreaForm from '$pages/areas/components/AreaForm.svelte'
	import { goto } from '$app/navigation'

	const { data } = $props()
	let area = $state(data.area)

	async function updateArea(e: Event) {
		e.preventDefault()
		if (!area) return
		try {
			await invoke('update_area_command', area.toUpdateDTO())
			goto(ROUTES.area.view(area.id))
		} catch (error) {
			console.error('Failed to update area:', error)
		}
	}
</script>

{#if area}
	<MainContainer title={`Editar ${area.name}`}>
		<AreaForm onsubmit={updateArea} bind:area>
			<div class="actions">
				<Button variant="secondary" outlined href={ROUTES.area.view(area.id)}>Cancelar</Button>
				<Button variant="primary" Icon={Save} type="submit">Guardar</Button>
			</div>
		</AreaForm>
	</MainContainer>
{/if}

<style>
	.actions {
		margin-top: 1rem;
		display: flex;
		justify-content: space-between;
	}
</style>
