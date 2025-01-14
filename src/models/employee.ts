import { EmployeeAssignment, type EmployeeAssignmentDTO } from '$models/employeeAssignment'

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
	id: number
	firstName: string
	lastName: string
	address: string
	createdAt: Date
	phone?: string
	startDate?: Date
	assignments: EmployeeAssignment[]

	constructor(params?: Partial<Omit<Employee, 'name' | 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		this.firstName = params?.firstName || ''
		this.lastName = params?.lastName || ''
		this.phone = params?.phone || ''
		this.address = params?.address || ''
		this.createdAt = params?.createdAt || new Date()
		this.startDate = params?.startDate || new Date()
		this.assignments = params?.assignments || []
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
			startDate: dto.start_date ? new Date(dto.start_date) : undefined,
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
			start_date: this.startDate?.toISOString(),
		}
	}

	public toUpdateDTO(): UpdateEmployeeDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}
