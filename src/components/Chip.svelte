<script lang="ts">
	interface Props {
		variant?: 'primary' | 'secondary'
		outlined?: boolean
		children: () => any
	}

	let { children, outlined, variant = 'primary' }: Props = $props()
</script>

<span class="chip {variant} {outlined ? 'outlined' : ''}">
	{@render children()}
</span>

<style>
	.chip {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 16px;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.primary {
		background-color: var(--primary-main);
		color: var(--primary-contrast);
	}

	.secondary {
		background-color: var(--secondary-main);
		color: var(--secondary-contrast);
	}

	.outlined {
		background-color: transparent;
		border: 1px solid currentColor;
	}

	.primary.outlined {
		color: var(--primary-main);
	}

	.secondary.outlined {
		color: var(--secondary-main);
	}
</style>
