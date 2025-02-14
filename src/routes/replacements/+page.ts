import { listReplacements } from '$queries/replacements'

export const load = async ({ url }) => {
	const employee_id = url.searchParams.get('employee_id')
	const replacements = await listReplacements(employee_id ? Number(employee_id) : null)
	return { replacements }
}
