<script lang="ts">
	import Select from '$components/Select.svelte'
	import { Employee } from '$models/employee'
	import { getEmployeeList } from '$queries/employees'
	import { onMount } from 'svelte'
	import type { HTMLSelectAttributes } from 'svelte/elements'

	let employees = $state<Employee[]>([])

	let { value = $bindable(), ...rest }: HTMLSelectAttributes = $props()

	onMount(async () => {
		employees = await getEmployeeList()
	})
</script>

<Select bind:value {...rest}>
	{#each employees as employee}
		<option value={employee.id}>{employee.name}</option>
	{/each}
</Select>
