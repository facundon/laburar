<script lang="ts">
	import { invalidateAll } from '$app/navigation'
	import Checkbox from '$components/Checkbox.svelte'
	import FormGroup from '$components/FormGroup.svelte'
	import Modal from '$components/Modal.svelte'
	import Rating from '$components/Rating.svelte'
	import { invoke } from '$invoke'
	import type { Employee } from '$models/employee'
	import type { EmployeeAssignment } from '$models/employeeAssignment'

	interface Props {
		onclose: () => void
		assignment: EmployeeAssignment
		employee: Employee
	}

	let { onclose, assignment, employee }: Props = $props()

	let efficiency = $state(assignment.efficiency)
	let isPrimary = $state(assignment.isPrimary)

	async function editAssignment() {
		try {
			await invoke('update_employee_assignment_command', {
				employee_id: employee.id,
				assignment_id: assignment.assignmentId,
				efficiency,
				is_primary: isPrimary,
			})
			invalidateAll()
			onclose()
		} catch (error) {
			console.error('Failed to edit assignment:', error)
		}
	}
</script>

<Modal show title="Editar {assignment.name} para {employee.name}" onconfirm={editAssignment} {onclose}>
	<Checkbox label="Es Primaria" id="isPrimary" bind:checked={isPrimary} style="margin-bottom: 2rem" />
	<FormGroup label="Eficiencia" id="efficiency">
		<Rating bind:rating={efficiency} isInteractive />
	</FormGroup>
</Modal>
