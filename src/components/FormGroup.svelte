<script lang="ts">
	let { children, id, label } = $props()
</script>

<div class="form-group">
	<label for={id}>{label}</label>
	{@render children()}
</div>

<style>
	.form-group {
		margin-bottom: 1rem;
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
	}
</style>
