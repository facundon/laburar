import { invoke } from '$invoke'
import { Absence, type AbsenceDTO } from '$models/absence'

export async function listAbsencesForEmployee(employeeId: number) {
	try {
		return invoke('list_absences_for_employee_command', { employee_id: employeeId }, Absence.fromDTO)
	} catch (error) {
		console.error('Failed to fetch area:', error)
		return null
	}
}

export async function listAbsences() {
	try {
		return invoke('list_absences_command', undefined, (data: AbsenceDTO[]) => data.map(Absence.fromDTO))
	} catch (error) {
		console.error('Failed to fetch area:', error)
		return null
	}
}
