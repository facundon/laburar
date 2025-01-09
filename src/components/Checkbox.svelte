<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements'

	let { label, id, color = 'primary', ...rest }: HTMLInputAttributes & { label: string; color?: 'primary' | 'secondary' } = $props()
</script>

<div class="checkbox-container">
	<input type="checkbox" class={color} {id} {...rest} />
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
