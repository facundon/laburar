<script lang="ts">
	import Button from '$components/Button.svelte'

	let { show = $bindable(false), title = '', message = '', onclose, onconfirm } = $props()

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
			<Button variant="error" onclick={confirm}>Confirm</Button>
		</div>
	</div>
{/if}

<style>
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
		padding: 2rem;
		border-radius: 8px;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
		z-index: 20;
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		margin-top: 1rem;
	}
</style>
