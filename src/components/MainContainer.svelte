<script lang="ts">
	import type { Snippet, SvelteComponent } from 'svelte'

	interface Props {
		title: string
		Actions?: Snippet
		style?: string
		children: () => any
	}

	let { title, Actions, style, children }: Props = $props()
</script>

<main class="container" {style}>
	<div class="header">
		<h1>{title}</h1>
		{#if Actions}<Actions />{/if}
	</div>
	{@render children()}
</main>

<style>
	.container {
		max-width: 1366px;
		margin: 0 auto;
		padding: 2rem;
		background-color: var(--gray-main);
		color: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.header {
		margin-bottom: 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
		color: #fff;
	}
</style>
