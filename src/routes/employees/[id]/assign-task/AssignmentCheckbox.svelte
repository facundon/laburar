<script lang="ts">
	import Checkbox from '$components/Checkbox.svelte'
	import type { Assignment } from '$models/assignment.svelte'

	interface Props {
		assignment: Assignment
		onchange(): void
	}

	let { assignment, onchange }: Props = $props()
	let isChecked = $state(false)
</script>

<Checkbox bind:checked={isChecked} label={`${assignment.taskName} - ${assignment.areaName}`} {onchange} />
