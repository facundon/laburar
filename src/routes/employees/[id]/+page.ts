import { listAbsencesForEmployee } from '$queries/absences'

export const load = async ({ params }) => {
	const employeeId = Number(params.id)
	const absences = await listAbsencesForEmployee(employeeId)
	return { absences }
}
