<script lang="ts">
	import type { FormEventHandler, HTMLTextareaAttributes } from 'svelte/elements'

	let { maxlength = 250, value = $bindable(), ...props }: HTMLTextareaAttributes = $props()
	let currentWords = $state(0)
	let currentLines = $state(1)

	const updateWordCount: FormEventHandler<HTMLTextAreaElement> = event => {
		if (!event.target || !('value' in event.target)) return
		const text = event.target.value as string
		currentWords = text.length
		currentLines = text.split('\n').length
	}
</script>

<textarea {value} oninput={updateWordCount} {...props} style={`height: ${currentLines * 20}px;`}></textarea>
<div class="word-counter" style={currentWords > (maxlength || 0) ? 'color: var(--error-light);' : ''}>
	{currentWords}/{maxlength}
</div>

<style>
	textarea {
		font-size: 1rem;
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 4px;
		resize: none;
		min-height: 1rem;
		max-height: 150px;
	}

	.word-counter {
		text-align: right;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: var(--secondary-light);
	}
</style>
