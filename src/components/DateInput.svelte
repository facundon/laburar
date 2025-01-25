<script lang="ts">
	import { formatDate, parseDate } from '$utils'
	import type { HTMLInputAttributes } from 'svelte/elements'

	interface Props extends HTMLInputAttributes {
		value: Date | undefined
		disabledDates?: string[]
	}

	let { value = $bindable(), disabledDates, ...rest }: Props = $props()

	function handleChange(event: Event) {
		const input = event.target as HTMLInputElement
		value = input.value ? parseDate(input.value, true) : undefined
	}

	let stringDate = $derived(value ? formatDate(value) : '')
	let isInvalid = $derived(disabledDates?.some(date => date === stringDate))
</script>

<input {...rest} type="date" value={stringDate} onchange={handleChange} />
{#if isInvalid}
	<p>Esa fecha es invalida</p>
{/if}

<style>
	p {
		color: var(--error-main);
		font-size: 0.9rem;
		margin-bottom: 0;
		margin-top: 0.25rem;
	}
</style>
