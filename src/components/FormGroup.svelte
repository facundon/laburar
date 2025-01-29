<script lang="ts">
	interface Props {
		id: string
		label: string
		children: () => any
	}

	let { children, id, label }: Props = $props()
</script>

<div class="form-group">
	<label for={id}>{label}</label>
	{@render children?.()}
</div>

<style>
	.form-group {
		margin-bottom: 1rem;
		width: 100%;
		flex: 1;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		font-weight: bold;
	}

	:global(.form-group input, select, textarea) {
		padding: 0.3rem;
		border: 2px solid var(--secondary-dark);
		border-radius: 4px;
		width: 100%;
		box-sizing: border-box;
	}

	:global(.form-group > .textarea-wrapper, .select-wrapper) {
		width: 100% !important;
	}
</style>
