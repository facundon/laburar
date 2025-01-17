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
	returnedHours: number
	returnDate: Date
	notes?: string
	createdAt?: Date

	constructor(params?: Partial<Omit<AbsenceReturn, 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		this.absenceId = params?.absenceId || 0
		this.returnedHours = params?.returnedHours || 0
		this.notes = params?.notes || ''
		this.createdAt = params?.createdAt || new Date()
		this.returnDate = params?.returnDate || new Date()
	}

	static fromDTO(dto: AbsenceReturnDTO): AbsenceReturn {
		return new AbsenceReturn({
			id: dto.id,
			absenceId: dto.absence_id,
			returnedHours: dto.returned_hours,
			notes: dto.notes,
			createdAt: dto.created_at ? new Date(dto.created_at) : undefined,
			returnDate: new Date(dto.return_date),
		})
	}

	public toCreateDTO(): CreateAbsenceReturnDTO {
		return {
			absence_id: this.absenceId,
			returned_hours: this.returnedHours,
			notes: this.notes,
			return_date: this.returnDate.toISOString(),
		}
	}

	public toUpdateDTO(): UpdateAbsenceReturnDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}
