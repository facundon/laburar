import { formatDate, parseDate } from '$utils'
import { SvelteDate } from 'svelte/reactivity'

type UpdateAbsenceReturnDTO = Omit<AbsenceReturnDTO, 'created_at'>
type CreateAbsenceReturnDTO = Omit<UpdateAbsenceReturnDTO, 'id'>

export type AbsenceReturnDTO = {
	id: number
	absence_id: number
	returned_hours: number
	return_date: string
	notes?: string
	created_at?: string
}

export class AbsenceReturn {
	id: number
	absenceId: number
	returnedHours: number = $state(1)
	returnDate: Date = $state(new SvelteDate())
	notes?: string = $state('')
	createdAt?: Date = new Date()

	constructor(params: Partial<Omit<AbsenceReturn, 'toCreateDTO' | 'toUpdateDTO'>> & { absenceId: number }) {
		this.id = params.id || 0
		this.absenceId = params.absenceId
		this.createdAt = params.createdAt
		if (params.returnedHours) this.returnedHours = params.returnedHours
		if (params.notes !== undefined) this.notes = params.notes
		if (params.returnDate !== undefined) this.returnDate = params.returnDate
	}

	static fromDTO(dto: AbsenceReturnDTO): AbsenceReturn {
		return new AbsenceReturn({
			id: dto.id,
			absenceId: dto.absence_id,
			returnedHours: dto.returned_hours,
			notes: dto.notes,
			createdAt: dto.created_at ? parseDate(dto.created_at) : undefined,
			returnDate: parseDate(dto.return_date, true),
		})
	}

	public toCreateDTO(): CreateAbsenceReturnDTO {
		return {
			absence_id: this.absenceId,
			returned_hours: this.returnedHours,
			notes: this.notes,
			return_date: formatDate(this.returnDate),
		}
	}

	public toUpdateDTO(): UpdateAbsenceReturnDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}
