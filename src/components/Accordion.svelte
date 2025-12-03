<script lang="ts">
	import { ChevronDown } from 'lucide-svelte'

	interface Props {
		title: string
		isOpen: boolean
		children?: () => any
	}

	let { title, isOpen = $bindable(), children }: Props = $props()

	function toggle() {
		isOpen = !isOpen
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault()
			toggle()
		}
	}
</script>

<div class="accordion">
	<button
		class="accordion-header"
		type="button"
		onclick={toggle}
		onkeydown={handleKeydown}
		aria-expanded={isOpen}
		aria-controls="accordion-content"
	>
		<span class="accordion-title">{title}</span>
		<ChevronDown class="chevron {isOpen ? 'open' : ''}" size={20} strokeWidth={1.5} />
	</button>
	{#if isOpen}
		<div class="accordion-content" id="accordion-content">
			{@render children?.()}
		</div>
	{/if}
</div>

<style>
	.accordion {
		border: 1px solid #ccc;
		border-radius: 0.25rem;
		margin-bottom: 0.5rem;
		overflow: hidden;
	}

	.accordion-header {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background-color: var(--background-main);
		border: none;
		cursor: pointer;
		text-align: left;
		transition: background-color 0.2s;
	}

	.accordion-header:hover {
		background-color: var(--primary-light);
	}

	.accordion-header:focus {
		outline: 2px solid var(--primary-main);
		outline-offset: -2px;
	}

	.accordion-title {
		font-weight: 500;
		font-size: 1rem;
		color: var(--gray-dark);
	}

	.accordion-content {
		padding: 1rem;
		background-color: white;
		border-top: 1px solid #ccc;
	}
</style>
