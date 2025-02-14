<script lang="ts">
	import Select from '$components/Select.svelte'
	import { Employee } from '$models/employee.svelte'
	import { getEmployeeList } from '$queries/employees'
	import { onMount } from 'svelte'
	import type { HTMLSelectAttributes } from 'svelte/elements'

	interface Props extends Omit<HTMLSelectAttributes, 'onchange' | 'value'> {
		value?: string | null
		onchange?: (value: string | null) => void
	}

	let employees = $state<Employee[]>([])

	let { value = $bindable(), ...rest }: Props = $props()

	onMount(async () => {
		employees = await getEmployeeList()
	})
</script>

<Select bind:value {...rest}>
	{#each employees as employee}
		<option value={employee.id}>{employee.name}</option>
	{/each}
</Select>
