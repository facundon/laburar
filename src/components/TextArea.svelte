<script lang="ts">
	import { onMount } from 'svelte'
	import type { HTMLTextareaAttributes } from 'svelte/elements'

	let { maxlength = 250, value = $bindable(), ...props }: HTMLTextareaAttributes = $props()
	let currentWords = $state(0)
	let currentLines = $state(1)

	const updateWordCount = (text: string) => {
		currentWords = text.length
		currentLines = text.split('\n').length
	}

	onMount(() => {
		if (value) updateWordCount(String(value))
	})
</script>

<div class="textarea-wrapper">
	<textarea
		bind:value
		oninput={e => updateWordCount((e.target as HTMLTextAreaElement)?.value)}
		{...props}
		style={`height: ${currentLines * 20}px;`}
	></textarea>
	<div class="word-counter" style={currentWords > (maxlength || 0) ? 'color: var(--error-light);' : ''}>
		{currentWords}/{maxlength}
	</div>
</div>

<style>
	textarea {
		width: 100%;
		border-radius: 4px;
		resize: none;
		min-height: 1rem;
		max-height: 150px;
	}

	.textarea-wrapper {
		width: fit-content;
		position: relative;
	}
	.word-counter {
		text-align: right;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: var(--secondary-light);
	}
</style>
