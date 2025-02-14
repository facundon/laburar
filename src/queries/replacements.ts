import { invoke } from '$invoke'
import { Replacement, type ReplacementDTO } from '$models/replacement.svelte'

export async function listReplacements(employee_id: number | null) {
	try {
		return invoke('list_replacements_command', { employee_id }, (data: ReplacementDTO[]) => data.map(Replacement.fromDTO))
	} catch (err) {
		console.error(`Failed to load replacements: ${err}`)
		return []
	}
}
