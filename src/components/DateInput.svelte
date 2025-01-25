<script lang="ts">
	import { formatDate, parseDate } from '$utils'
	import type { HTMLInputAttributes } from 'svelte/elements'

	interface Props extends HTMLInputAttributes {
		value: Date | undefined
	}

	let { value = $bindable(), ...rest }: Props = $props()

	function handleChange(event: Event) {
		const input = event.target as HTMLInputElement
		value = input.value ? parseDate(input.value, true) : undefined
	}

	let stringDate = $derived(value ? formatDate(value) : '')
</script>

<input {...rest} type="date" value={stringDate} onchange={handleChange} />
