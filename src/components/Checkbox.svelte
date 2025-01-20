<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements'

	let {
		label,
		id,
		color = 'primary',
		checked = $bindable(),
		value = $bindable(),
		style,
		...rest
	}: HTMLInputAttributes & { label: string; color?: 'primary' | 'secondary' } = $props()
</script>

<div class="checkbox-container" {style}>
	<input type="checkbox" class={color} {id} bind:checked {...rest} />
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<label
		for={id}
		aria-label={label}
		onkeydown={e => e.key === 'Enter' && (e.target as HTMLInputElement).click()}
		onclick={() => {
			checked = !checked
			const changeEvent = new Event('change') as Event & { currentTarget: EventTarget & HTMLInputElement }
			rest.onchange?.(changeEvent)
		}}>{label}</label
	>
</div>

<style>
	.checkbox-container {
		display: flex;
		align-items: center;
		margin-bottom: 1rem;
	}

	input[type='checkbox'] {
		margin-right: 0.8rem;
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
