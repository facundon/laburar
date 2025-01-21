import { invoke } from '$invoke'
import { Absence, type AbsenceDTO } from '$models/absence.svelte'

export async function listAbsencesForEmployee(employeeId: number) {
	try {
		return invoke('list_absences_for_employee_command', { employee_id: employeeId }, (data: AbsenceDTO[]) => data.map(Absence.fromDTO))
	} catch (error) {
		console.error('Failed to fetch absence:', error)
		return null
	}
}

export async function listAbsences() {
	try {
		return invoke('list_absences_command', undefined, (data: AbsenceDTO[]) => data.map(Absence.fromDTO))
	} catch (error) {
		console.error('Failed to fetch absence:', error)
		return null
	}
}

export async function getAbsenceWithReturns(id: number) {
	try {
		return invoke('get_absence_with_returns_command', { id }, Absence.fromDTO)
	} catch (error) {
		console.error('Failed to fetch absence:', error)
		return null
	}
}
