import { formatDate, parseDate } from '$utils'
import { SvelteDate } from 'svelte/reactivity'

type UpdateHolidayDTO = Omit<HolidayDTO, 'created_at' | 'employee_name'>
type CreateHolidayDTO = Omit<UpdateHolidayDTO, 'id'>

export type HolidayDTO = {
	id: number
	employee_id: number
	employee_name: string
	start_date: string
	end_date: string
	days_off: number
	notes?: string
	created_at: string
}

export class Holiday {
	id: number = 0
	employeeId: number = $state(0)
	employeeName: string = ''
	startDate: Date = new SvelteDate()
	endDate: Date = new SvelteDate()
	daysOff: number = $state(1)
	notes: string = $state('')
	createdAt: Date = new Date()

	constructor(params?: Partial<Omit<Holiday, 'toCreateDTO' | 'toUpdateDTO'>>) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.employeeName !== undefined) this.employeeName = params.employeeName
		if (params?.employeeId !== undefined) this.employeeId = params.employeeId
		if (params?.startDate !== undefined) this.startDate = params.startDate
		if (params?.endDate !== undefined) this.endDate = params.endDate
		if (params?.daysOff !== undefined) this.daysOff = params.daysOff
		if (params?.notes !== undefined) this.notes = params.notes
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
	}

	static fromDTO(dto: HolidayDTO): Holiday {
		return new Holiday({
			id: dto.id,
			employeeId: dto.employee_id,
			employeeName: dto.employee_name,
			startDate: parseDate(dto.start_date, true),
			endDate: parseDate(dto.end_date, true),
			daysOff: dto.days_off,
			notes: dto.notes,
			createdAt: parseDate(dto.created_at),
		})
	}

	public toCreateDTO(): CreateHolidayDTO {
		return {
			employee_id: this.employeeId,
			start_date: formatDate(this.startDate),
			end_date: formatDate(this.endDate),
			days_off: this.daysOff,
			notes: this.notes,
		}
	}

	public toUpdateDTO(): UpdateHolidayDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}
