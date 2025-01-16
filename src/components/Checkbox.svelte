<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements'

	let {
		label,
		id,
		color = 'primary',
		checked = $bindable(),
		style,
		...rest
	}: HTMLInputAttributes & { label: string; color?: 'primary' | 'secondary' } = $props()

	function handleChange(e: Event) {
		checked = (e.target as HTMLInputElement).checked
	}
</script>

<div class="checkbox-container" {style}>
	<input type="checkbox" class={color} {id} {checked} onchange={handleChange} {...rest} />
	<label for={id}>{label}</label>
</div>

<style>
	.checkbox-container {
		display: flex;
		align-items: center;
	}

	input[type='checkbox'] {
		margin-right: 10px;
		transform: scale(1.2);
	}

	input.primary[type='checkbox'] {
		accent-color: var(--primary-main);
	}

	input.secondary[type='checkbox'] {
		accent-color: var(--secondary-main);
	}

	label {
		user-select: none;
		cursor: pointer;
	}
</style>
