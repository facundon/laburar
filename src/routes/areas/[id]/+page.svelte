<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import Button from '$components/Button.svelte'
	import Modal from '$components/Modal.svelte'
	import { Delete, Pencil } from 'lucide-svelte'
	import MainContainer from '$components/MainContainer.svelte'

	const { data } = $props()
	const area = data.area

	let showModal = $state(false)

	async function deleteArea() {
		try {
			if (!area) return
			await invoke('delete_area_command', { id: area.id })
			window.location.href = ROUTES.area.list
		} catch (error) {
			console.error('Failed to delete area:', error)
		}
	}

	const openModal = () => (showModal = true)
	const closeModal = () => (showModal = false)

	function handleConfirm() {
		deleteArea()
		closeModal()
	}
</script>

{#if area}
	<MainContainer title={area.name}>
		<strong>Descripción:</strong>
		<p class="area">{area.description}</p>

		<div class="actions">
			<Button outlined href={ROUTES.area.edit(area.id)} Icon={Pencil}>Editar</Button>
			<Button outlined variant="error" onclick={openModal} Icon={Delete}>Eliminar</Button>
		</div>
		<Modal
			bind:show={showModal}
			title="Confirmar acción"
			isDestructive
			message={`¿Estás seguro de que deseas eliminar la area "${area.name}"?`}
			onconfirm={handleConfirm}
			onclose={closeModal}
		/>
	</MainContainer>
{/if}

<style>
	p.area {
		margin-bottom: 0.5rem;
		color: #fff;
		white-space: pre-wrap;
	}
	.actions {
		margin-top: 3rem;
		display: flex;
		justify-content: space-between;
	}
</style>
