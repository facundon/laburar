import { listAbsencesForEmployee } from '$queries/absences'
import { listHolidaysForEmployee } from '$queries/holidays'

export const load = async ({ params }) => {
	const employeeId = Number(params.id)
	const absences = await listAbsencesForEmployee(employeeId)
	const holidays = await listHolidaysForEmployee(employeeId)
	return { absences, holidays }
}
