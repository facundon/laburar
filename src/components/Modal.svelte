<script lang="ts">
	import Button from '$components/Button.svelte'
	import { Check, Trash2 } from 'lucide-svelte'

	let { show = $bindable(false), title = '', message = '', onclose, onconfirm, isDestructive = false } = $props()

	function close() {
		onclose()
	}

	function confirm() {
		onconfirm()
	}
</script>

{#if show}
	<div class="modal-backdrop" role="button" tabindex="0" onclick={close} onkeydown={e => e.key === 'Enter' && close()}></div>
	<div class="modal">
		<h2>{title}</h2>
		<p>{message}</p>
		<div class="actions">
			<Button onclick={close}>Cancel</Button>
			<Button variant={isDestructive ? 'error' : 'primary'} onclick={confirm} Icon={isDestructive ? Trash2 : Check}>Confirm</Button>
		</div>
	</div>
{/if}

<style>
	p {
		white-space: pre-wrap;
		line-height: 2rem;
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
		padding-inline: 1.5rem;
		padding-bottom: 1rem;
		border-radius: 8px;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
		z-index: 20;
		max-width: clamp(20rem, 50vw, 45rem);
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 1rem;
	}
</style>
