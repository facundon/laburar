<script lang="ts">
	import { ChevronDown } from 'lucide-svelte'
	import type { HTMLSelectAttributes } from 'svelte/elements'

	let { children, value = $bindable(), ...rest }: HTMLSelectAttributes = $props()

	let select: HTMLSelectElement

	function handleButtonClick(e: Event) {
		select.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, cancelable: true, view: window }))
	}
</script>

<div class="select-wrapper">
	<select {...rest} bind:this={select} bind:value>
		{@render children?.()}
	</select>
	<button class={`icon`} onclick={handleButtonClick} type="button" tabindex="-1">
		<ChevronDown color="var(--secondary-dark)" strokeWidth={1} />
	</button>
</div>

<style>
	.select-wrapper {
		width: fit-content;
		position: relative;
	}
	select {
		width: 100%;
		padding-right: 2rem;
	}
	.icon {
		background: transparent;
		padding: 0;
		margin: 0;
		border: none;
		appearance: none;
		position: absolute;
		right: 5px;

		top: 50%;
		transform: translateY(-42%);
	}
</style>
