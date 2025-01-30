import { breadcrumbData } from '$context'
import { getAbsenceWithReturns } from '$queries/absences'

export const prerender = false

export const load = async ({ params }) => {
	const id = Number(params.id)
	const absence = await getAbsenceWithReturns(id)
	if (absence) breadcrumbData.name = `${absence?.absenceDate.toLocaleDateString()} - ${absence?.employeeName}`
	return { absence }
}
