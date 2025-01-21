<script lang="ts">
	import { invalidateAll } from '$app/navigation'
	import Checkbox from '$components/Checkbox.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import Modal from '$components/Modal.svelte'
	import Rating from '$components/Rating.svelte'
	import { invoke } from '$invoke'
	import type { Employee } from '$models/employee.svelte'
	import { EmployeeAssignment } from '$models/employeeAssignment.svelte'

	interface Props {
		onclose: () => void
		assignment: EmployeeAssignment
		employee: Employee
	}

	let { onclose, assignment = $bindable(), employee }: Props = $props()
	const newAssigment = new EmployeeAssignment(assignment)

	async function editAssignment() {
		try {
			await invoke('update_employee_assignment_command', newAssigment.toUpdateDTO())
			invalidateAll()
			onclose()
		} catch (error) {
			console.error('Failed to edit assignment:', error)
		}
	}
</script>

<Modal show title="Editar {assignment.name} para {employee.name}" onconfirm={editAssignment} {onclose}>
	<Checkbox label="Es Primaria" bind:checked={newAssigment.isPrimary} style="margin-bottom: 2rem" />
	<FormGroup label="Eficiencia" id="efficiency">
		<Rating bind:rating={newAssigment.efficiency} isInteractive />
	</FormGroup>
</Modal>
