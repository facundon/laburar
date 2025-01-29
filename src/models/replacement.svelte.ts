import { formatDate, parseDate } from '$utils'
import { SvelteDate } from 'svelte/reactivity'

type UpdateReplacementDTO = Omit<ReplacementDTO, 'created_at' | 'assignment' | 'original_employee_name' | 'replacement_employee_name'>
type CreateReplacementDTO = Omit<UpdateReplacementDTO, 'id'>

export type ReplacementDTO = {
	id: number
	original_employee_id: number
	original_employee_name: string
	replacement_employee_id: number
	replacement_employee_name: string
	replacement_start_date: string
	replacement_end_date: string
	assignment_id: number
	assignment: string
	notes?: string
	created_at: string
}

export class Replacement {
	id: number = 0
	originalEmployeeId: number = $state(0)
	replacementEmployeeId: number = $state(0)
	replacementStartDate: Date = $state(new SvelteDate())
	replacementEndDate: Date = $state(new SvelteDate())
	assignmentId: number = $state(0)
	notes?: string = $state('')
	createdAt: Date = new Date()
	assignment: string = ''
	originalEmployeeName: string = ''
	replacementEmployeeName: string = ''

	constructor(params?: Partial<Omit<Replacement, 'toCreateDTO' | 'toUpdateDTO'>>) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.originalEmployeeId !== undefined) this.originalEmployeeId = params.originalEmployeeId
		if (params?.replacementEmployeeId !== undefined) this.replacementEmployeeId = params.replacementEmployeeId
		if (params?.replacementStartDate !== undefined) this.replacementStartDate = params.replacementStartDate
		if (params?.replacementEndDate !== undefined) this.replacementEndDate = params.replacementEndDate
		if (params?.assignmentId !== undefined) this.assignmentId = params.assignmentId
		if (params?.notes !== undefined) this.notes = params.notes
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
		if (params?.assignment !== undefined) this.assignment = params.assignment
		if (params?.originalEmployeeName !== undefined) this.originalEmployeeName = params.originalEmployeeName
		if (params?.replacementEmployeeName !== undefined) this.replacementEmployeeName = params.replacementEmployeeName
	}

	static fromDTO(dto: ReplacementDTO): Replacement {
		return new Replacement({
			id: dto.id,
			originalEmployeeId: dto.original_employee_id,
			replacementEmployeeId: dto.replacement_employee_id,
			replacementStartDate: parseDate(dto.replacement_start_date, true),
			replacementEndDate: parseDate(dto.replacement_end_date, true),
			assignmentId: dto.assignment_id,
			notes: dto.notes,
			createdAt: parseDate(dto.created_at),
			assignment: dto.assignment,
			originalEmployeeName: dto.original_employee_name,
			replacementEmployeeName: dto.replacement_employee_name,
		})
	}

	public toCreateDTO(): CreateReplacementDTO {
		return {
			original_employee_id: this.originalEmployeeId,
			replacement_employee_id: this.replacementEmployeeId,
			replacement_start_date: formatDate(this.replacementStartDate),
			replacement_end_date: formatDate(this.replacementEndDate),
			assignment_id: this.assignmentId,
			notes: this.notes,
		}
	}

	public toUpdateDTO(): UpdateReplacementDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}
