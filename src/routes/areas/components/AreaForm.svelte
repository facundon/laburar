<script lang="ts">
	import FormGroup from '$components/FormGroup.svelte'
	import TextArea from '$components/TextArea.svelte'
	import { Area } from '$models/area'

	interface Props {
		area: Area
		onsubmit: (event: Event) => void
		children: () => any
	}

	let { area = $bindable(), onsubmit = $bindable(), children }: Props = $props()
</script>

<form {onsubmit}>
	<div class="group">
		<FormGroup label="Nombre" id="name">
			<input id="name" bind:value={area.name} required />
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup label="Descripción" id="description">
			<TextArea id="description" bind:value={area.description}></TextArea>
		</FormGroup>
	</div>
	{@render children()}
</form>

<style>
	.group {
		display: flex;
		gap: 3rem;
	}
</style>
