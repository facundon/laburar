import { EmployeeAssignment, type EmployeeAssignmentDTO } from '$models/employeeAssignment'
import { format } from 'date-fns'
import { SvelteDate } from 'svelte/reactivity'

type UpdateEmployeeDTO = Omit<EmployeeDTO, 'created_at' | 'assignments'>
type CreateEmployeeDTO = Omit<UpdateEmployeeDTO, 'id'>

export type EmployeeDTO = {
	id: number
	first_name: string
	last_name: string
	address: string
	phone?: string
	start_date?: string
	created_at: string
	assignments: EmployeeAssignmentDTO[]
}

export class Employee {
	id: number = 0
	firstName: string = $state('')
	lastName: string = $state('')
	address: string = $state('')
	createdAt: Date = new Date()
	phone?: string = $state('')
	startDate?: Date = new SvelteDate()
	assignments: EmployeeAssignment[] = $state([])

	constructor(params?: Partial<Omit<Employee, 'name' | 'toCreateDTO' | 'toUpdateDTO'>>) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.firstName !== undefined) this.firstName = params.firstName
		if (params?.lastName !== undefined) this.lastName = params.lastName
		if (params?.phone !== undefined) this.phone = params.phone
		if (params?.address !== undefined) this.address = params.address
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
		if (params?.startDate !== undefined) this.startDate = params.startDate
		if (params?.assignments !== undefined) this.assignments = params.assignments
	}

	get name() {
		return `${this.firstName} ${this.lastName}`
	}

	static fromDTO(dto: EmployeeDTO): Employee {
		return new Employee({
			id: dto.id,
			firstName: dto.first_name,
			lastName: dto.last_name,
			address: dto.address,
			phone: dto.phone,
			startDate: dto.start_date ? new SvelteDate(dto.start_date) : undefined,
			createdAt: new Date(dto.created_at),
			assignments: dto.assignments?.map(EmployeeAssignment.fromDTO),
		})
	}

	public toCreateDTO(): CreateEmployeeDTO {
		return {
			first_name: this.firstName,
			last_name: this.lastName,
			address: this.address,
			phone: this.phone,
			start_date: this.startDate ? format(this.startDate, 'yyyy-MM-dd') : undefined,
		}
	}

	public toUpdateDTO(): UpdateEmployeeDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}
