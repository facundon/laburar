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

	:global(.wrapper > h3) {
		color: var(--primary-main);
	}

	:global(.wrapper > p) {
		text-align: center;
		margin-block: 3rem;
	}

	:global(p) {
		font-weight: 500;
	}

	:global(p > span) {
		font-weight: 600;
		color: var(--secondary-light);
	}
</style>
