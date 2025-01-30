<script lang="ts">
	import Button from '$components/Button.svelte'
	import { Check, Trash2 } from 'lucide-svelte'

	interface Props {
		show: boolean
		title?: string
		message?: string
		isDestructive?: boolean
		disableCta?: boolean
		onclose: () => void
		onconfirm: () => void
		onmount?: () => void
		children?: () => any
	}

	let {
		show = $bindable(false),
		title = 'Confirmar Acción',
		message = '',
		onclose,
		onconfirm,
		isDestructive = false,
		children,
		onmount,
		disableCta = false,
	}: Props = $props()

	function close() {
		onclose()
	}

	function confirm() {
		onconfirm()
	}

	$effect(() => {
		if (show) onmount?.()
	})
</script>

{#if show}
	<div class="modal-backdrop" role="button" tabindex="0" onclick={close} onkeydown={e => e.key === 'Enter' && close()}></div>
	<div class="modal">
		<h2>{title}</h2>
		<div class="content">
			{#if message}
				<p>{@html message}</p>
			{/if}
			{@render children?.()}
		</div>
		<div class="actions">
			<Button onclick={close} variant="secondary-dark" outlined>Cancelar</Button>
			<Button variant={isDestructive ? 'error' : 'primary'} onclick={confirm} Icon={isDestructive ? Trash2 : Check} disabled={disableCta}
				>Confirmar</Button
			>
		</div>
	</div>
{/if}

<style>
	h2 {
		padding-inline: 1.5rem;
	}
	p {
		margin: 0;
		white-space: pre-wrap;
		line-height: 2rem;
	}

	.content {
		overflow-y: auto;
		padding-block: 1rem;
		padding-inline: 1.5rem;
		height: fit-content;
		max-height: 60vh;
	}
	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		z-index: 10;
	}

	.modal {
		position: fixed;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: white;
		color: #333;
		padding-bottom: 1rem;
		border-radius: 8px;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
		z-index: 20;
		width: max-content;
		max-width: clamp(40rem, 80vw, 180rem);
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 1rem;
		padding-inline: 1.5rem;
	}
</style>
