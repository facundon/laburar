<script lang="ts">
	let { children, title } = $props()
</script>

<div class="wrapper">
	<h2>{title}</h2>
	{@render children?.()}
</div>

<style>
	.wrapper {
		color: #fff;
	}

	h2 {
		margin-top: 0.5rem;
	}

	.wrapper :global(.wrapper > h3) {
		color: var(--primary-main);
	}

	.wrapper :global(p) {
		font-weight: 500;
	}

	.wrapper :global(p > span) {
		font-weight: 600;
		color: var(--secondary-light);
	}

	.wrapper :global(.empty) {
		text-align: center;
		margin-block: 3rem;
	}
</style>
