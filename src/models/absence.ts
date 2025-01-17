import { AbsenceReturn, type AbsenceReturnDTO } from '$models/absenceReturn'

type UpdateAbsenceDTO = Omit<AbsenceDTO, 'created_at' | 'returns'>
type CreateAbsenceDTO = Omit<UpdateAbsenceDTO, 'id'>

export type AbsenceDTO = {
	id: number
	employee_id: number
	is_justified: boolean
	will_return: boolean
	hours: number
	description?: string
	absence_type: string
	absence_date: string
	created_at?: string
	returns: AbsenceReturnDTO[]
}

export class Absence {
	id: number
	employeeId: number
	isJustified: boolean
	willReturn: boolean
	hours: number
	description?: string
	absenceType: string
	absenceDate: Date
	createdAt?: Date
	returns: AbsenceReturn[]

	constructor(params?: Partial<Omit<Absence, 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		this.employeeId = params?.employeeId || 0
		this.isJustified = params?.isJustified || false
		this.willReturn = params?.willReturn || false
		this.hours = params?.hours || 0
		this.description = params?.description || ''
		this.absenceType = params?.absenceType || ''
		this.absenceDate = params?.absenceDate || new Date()
		this.createdAt = params?.createdAt || new Date()
		this.returns = params?.returns || []
	}

	static fromDTO(dto: AbsenceDTO): Absence {
		return new Absence({
			id: dto.id,
			employeeId: dto.employee_id,
			isJustified: dto.is_justified,
			willReturn: dto.will_return,
			hours: dto.hours,
			description: dto.description,
			absenceType: dto.absence_type,
			absenceDate: new Date(dto.absence_date),
			createdAt: dto.created_at ? new Date(dto.created_at) : undefined,
			returns: dto.returns?.map(AbsenceReturn.fromDTO),
		})
	}

	public toCreateDTO(): CreateAbsenceDTO {
		return {
			employee_id: this.employeeId,
			is_justified: this.isJustified,
			will_return: this.willReturn,
			hours: this.hours,
			description: this.description,
			absence_type: this.absenceType,
			absence_date: this.absenceDate.toISOString(),
		}
	}

	public toUpdateDTO(): UpdateAbsenceDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}
