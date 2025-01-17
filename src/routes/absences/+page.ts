import { listAbsences } from '$queries/absences'

export const load = async () => {
	const absences = await listAbsences()
	return { absences }
}
