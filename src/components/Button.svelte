<script lang="ts">
	import type { HTMLButtonAttributes } from 'svelte/elements'
	import { goto } from '$app/navigation'
	import type { Icon as IconType } from 'lucide-svelte'

	type LinkProps = {
		href: string
		onclick?: never
	}
	type ButtonProps = {
		href?: never
		onclick?: HTMLButtonAttributes['onclick']
	}

	type Props = Omit<HTMLButtonAttributes, 'onclick'> & {
		outlined?: boolean
		fullWidth?: boolean
		Icon?: typeof IconType
		variant?: 'primary' | 'secondary' | 'error'
	} & (ButtonProps | LinkProps)

	let { children, href, Icon, variant: type = 'primary', fullWidth = false, outlined = false, ...rest }: Props = $props()
</script>

<button
	class="button {type} {outlined ? 'outlined' : ''} {fullWidth ? 'fullwidth' : ''}"
	onclick={href ? () => goto(href) : onclick}
	{...rest}
>
	{#if Icon}
		<Icon style="margin-right: 8px;" strokeWidth={1.5} />
	{/if}
	{@render children?.()}
</button>

<style>
	.button {
		align-items: center;
		display: flex;
		padding: 0.5rem 1rem;
		border-radius: 4px;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 500;
		text-align: center;
		text-decoration: none;
		transition:
			background-color 0.3s,
			color 0.3s,
			border-color 0.3s;
	}

	.fullwidth {
		flex-grow: 1;
		width: 100%;
	}

	.button.primary {
		background-color: var(--primary-main);
		color: var(--primary-contrast);
		border: 1px solid var(--primary-main);
	}

	.button.primary.outlined {
		background-color: transparent;
		color: var(--primary-main);
		border: 1px solid var(--primary-main);
	}

	.button.secondary {
		background-color: var(--secondary-main);
		color: var(--secondary-contrast);
		border: 1px solid var(--secondary-main);
	}

	.button.secondary.outlined {
		background-color: transparent;
		color: var(--secondary-main);
		border: 1px solid var(--secondary-main);
	}

	.button.error {
		background-color: var(--error-main);
		color: var(--error-contrast);
		border: 1px solid var(--error-main);
	}

	.button.error.outlined {
		background-color: transparent;
		color: var(--error-main);
		border: 1px solid var(--error-main);
	}

	.button:hover:not(.outlined):not(:disabled) {
		background-color: var(--primary-dark);
		border-color: var(--primary-dark);
	}

	.button.primary.outlined:hover:not(:disabled) {
		background-color: var(--primary-main);
		color: var(--primary-contrast);
	}

	.button.secondary.outlined:hover:not(:disabled) {
		background-color: var(--secondary-main);
		color: var(--secondary-contrast);
	}

	.button.secondary:hover:not(.outlined):not(:disabled) {
		background-color: var(--secondary-dark);
	}

	.button.error:hover:not(.outlined):not(:disabled) {
		background-color: var(--error-dark);
	}

	.button.error.outlined:hover:not(:disabled) {
		background-color: var(--error-main);
		color: var(--error-contrast);
	}

	.button:disabled {
		cursor: not-allowed;
		opacity: 0.6;
	}
</style>
