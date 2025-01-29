import { listReplacements } from '$queries/replacements'

export const load = async () => {
	const replacements = await listReplacements()
	return { replacements }
}
