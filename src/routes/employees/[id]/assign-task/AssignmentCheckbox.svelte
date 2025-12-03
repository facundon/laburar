<script lang="ts">
	import Checkbox from '$components/Checkbox.svelte'
	import type { Assignment } from '$models/assignment.svelte'

	interface Props {
		assignment: Assignment
		checked?: boolean
		onchange(): void
	}

	let { assignment, checked = $bindable(false), onchange }: Props = $props()
</script>

<Checkbox bind:checked label={assignment.taskName || ''} {onchange} />
