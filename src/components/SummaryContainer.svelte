<script lang="ts">
	interface Props {
		title: string
		Action?: () => any
		children?: () => any
	}

	let { children, title, Action }: Props = $props()
</script>

<div class="wrapper">
	<div class="title-wrapper">
		<h2>{title}</h2>
		{@render Action?.()}
	</div>
	{@render children?.()}
</div>

<style>
	.title-wrapper {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}
	.wrapper {
		color: #fff;
	}

	h2 {
		margin-block: 0;
	}

	.wrapper :global(> h3) {
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
