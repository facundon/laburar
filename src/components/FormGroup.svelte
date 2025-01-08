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

	:global(.form-group input) {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 4px;
	}
</style>
