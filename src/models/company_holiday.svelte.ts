import { formatDate, parseDate } from '$utils'
import { SvelteDate } from 'svelte/reactivity'

type UpdateCompanyHolidayDTO = Omit<CompanyHolidayDTO, 'created_at'>
type CreateCompanyHolidayDTO = Omit<UpdateCompanyHolidayDTO, 'id'>

export type CompanyHolidayDTO = {
	id: number
	date: string
	description?: string
	created_at: string
}

export class CompanyHoliday {
	id: number = 0
	date: Date = $state(new SvelteDate())
	description: string = $state('')
	createdAt: Date = new Date()

	constructor(params?: Partial<Omit<CompanyHoliday, 'toCreateDTO' | 'toUpdateDTO'>>) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.date !== undefined) this.date = params.date
		if (params?.description !== undefined) this.description = params.description
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
	}

	static fromDTO(dto: CompanyHolidayDTO): CompanyHoliday {
		return new CompanyHoliday({
			id: dto.id,
			date: parseDate(dto.date, true),
			description: dto.description,
			createdAt: parseDate(dto.created_at),
		})
	}

	public toCreateDTO(): CreateCompanyHolidayDTO {
		return {
			date: formatDate(this.date),
			description: this.description,
		}
	}

	public toUpdateDTO(): UpdateCompanyHolidayDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}
