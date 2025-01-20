<script lang="ts">
	import type { ChangeEventHandler, HTMLInputAttributes } from 'svelte/elements'

	let { value = $bindable(), ...rest }: HTMLInputAttributes = $props()

	function handleKeyPress(
		event: KeyboardEvent & {
			currentTarget: EventTarget & HTMLInputElement
		},
	) {
		if (!/[0-9]/.test(event.key)) event.preventDefault()
		rest.onkeypress?.(event)
	}
</script>

<input type="number" {value} onkeypress={handleKeyPress} {...rest} onchange={e => (value = Number((e.target as HTMLInputElement).value))} />
