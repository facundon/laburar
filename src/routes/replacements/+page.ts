import { listReplacements } from '$queries/replacements'

export const load = async ({ url }) => {
	const employee_id = url.searchParams.get('employee_id')
	const show_old_only = Boolean(url.searchParams.get('show_old_only'))
	const replacements = await listReplacements(employee_id ? Number(employee_id) : null, show_old_only)
	return { replacements }
}
