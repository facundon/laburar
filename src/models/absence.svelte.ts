import { AbsenceReturn, type AbsenceReturnDTO } from '$models/absenceReturn.svelte'
import { formatDate, parseDate } from '$utils'
import { SvelteDate } from 'svelte/reactivity'

type UpdateAbsenceDTO = Omit<AbsenceDTO, 'created_at' | 'returns' | 'is_returned' | 'employee_name'>
type CreateAbsenceDTO = Omit<UpdateAbsenceDTO, 'id'>

export type AbsenceDTO = {
	id: number
	employee_id: number
	employee_name: string
	is_justified: boolean
	is_returned: boolean
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
	employeeId: number = $state(0)
	isJustified: boolean = $state(false)
	willReturn: boolean = $state(false)
	hours: number = $state(4)
	description?: string = $state('')
	absenceType: string = $state('')
	isReturned: boolean = false
	absenceDate: Date = $state(new SvelteDate())
	createdAt?: Date
	employeeName: string = ''
	returns: AbsenceReturn[] = $state([])

	constructor(params?: Partial<Omit<Absence, 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		if (params?.employeeId) this.employeeId = params.employeeId
		if (params?.isJustified !== undefined) this.isJustified = params.isJustified
		if (params?.willReturn !== undefined) this.willReturn = params.willReturn
		if (params?.isReturned !== undefined) this.isReturned = params.isReturned
		if (params?.hours !== undefined) this.hours = params.hours
		if (params?.description !== undefined) this.description = params.description
		if (params?.absenceType !== undefined) this.absenceType = params.absenceType
		if (params?.absenceDate !== undefined) this.absenceDate = params.absenceDate
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
		if (params?.returns !== undefined) this.returns = params.returns
		if (params?.employeeName !== undefined) this.employeeName = params.employeeName
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
			absenceDate: parseDate(dto.absence_date, true),
			createdAt: dto.created_at ? parseDate(dto.created_at) : undefined,
			returns: dto.returns?.map(AbsenceReturn.fromDTO),
			isReturned: dto.is_returned,
			employeeName: dto.employee_name,
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
			absence_date: formatDate(this.absenceDate),
		}
	}

	public toUpdateDTO(): UpdateAbsenceDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}

export const AbsenceType = {
	ENFERMO: 'Enfermo',
	ESTUDIO: 'Estudio',
	FAMILIAR: 'Familiar',
	MOTIVO_PERSONAL: 'Motivo Personal',
	OTRO: 'Otro',
} as const
export type AbsenceType = ValueOf<typeof AbsenceType>
export const AbsenceTypes = Object.entries(AbsenceType).map(([_, value]) => ({ label: value, value }))
