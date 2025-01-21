<script lang="ts">
	import { format } from 'date-fns'
	import type { HTMLInputAttributes } from 'svelte/elements'

	interface Props extends HTMLInputAttributes {
		value: Date | undefined
	}

	let { value = $bindable(), ...rest }: Props = $props()

	function handleChange(event: Event) {
		const input = event.target as HTMLInputElement
		value = input.value ? new Date(input.value) : undefined
	}
</script>

<input {...rest} type="date" value={value ? format(value, 'yyyy-MM-dd') : ''} onchange={handleChange} />
