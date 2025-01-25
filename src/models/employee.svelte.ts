import { EmployeeAssignment, type EmployeeAssignmentDTO } from '$models/employeeAssignment.svelte'
import { formatDate, parseDate } from '$utils'
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
	startDate?: Date = $state(new SvelteDate())
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
			startDate: dto.start_date ? parseDate(dto.start_date, true) : undefined,
			createdAt: parseDate(dto.created_at),
			assignments: dto.assignments?.map(EmployeeAssignment.fromDTO),
		})
	}

	public toCreateDTO(): CreateEmployeeDTO {
		return {
			first_name: this.firstName,
			last_name: this.lastName,
			address: this.address,
			phone: this.phone,
			start_date: this.startDate ? formatDate(this.startDate) : undefined,
		}
	}

	public toUpdateDTO(): UpdateEmployeeDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}

export interface SuggestedEmployeeDTO {
	id: number
	first_name: string
	last_name: string
	efficiency: number
	task_difficulty: number
	is_primary: boolean
	start_date: string
	end_date: string
	assignments_difficulties: number[]
	score: number
}

export class SuggestedEmployee {
	id: number = 0
	firstName: string = ''
	lastName: string = ''
	efficiency: number = 0
	taskDifficulty: number = 0
	isPrimary: boolean = false
	startDate: Date | null = null
	endDate: Date | null = null
	assignmentsDifficulties: number[] = []
	score: number = 0

	constructor(params?: Partial<Omit<SuggestedEmployee, 'name'>>) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.firstName !== undefined) this.firstName = params.firstName
		if (params?.lastName !== undefined) this.lastName = params.lastName
		if (params?.efficiency !== undefined) this.efficiency = params.efficiency
		if (params?.taskDifficulty !== undefined) this.taskDifficulty = params.taskDifficulty
		if (params?.isPrimary !== undefined) this.isPrimary = params.isPrimary
		if (params?.startDate !== undefined) this.startDate = params.startDate
		if (params?.endDate !== undefined) this.endDate = params.endDate
		if (params?.assignmentsDifficulties !== undefined) this.assignmentsDifficulties = params.assignmentsDifficulties
		if (params?.score !== undefined) this.score = params.score
	}

	get name() {
		return `${this.firstName} ${this.lastName}`
	}

	static fromDTO(dto: SuggestedEmployeeDTO): SuggestedEmployee {
		return new SuggestedEmployee({
			id: dto.id,
			firstName: dto.first_name,
			lastName: dto.last_name,
			efficiency: dto.efficiency,
			taskDifficulty: dto.task_difficulty,
			isPrimary: dto.is_primary,
			startDate: dto.start_date ? parseDate(dto.start_date) : null,
			endDate: dto.end_date ? parseDate(dto.end_date) : null,
			assignmentsDifficulties: dto.assignments_difficulties,
			score: dto.score,
		})
	}
}

export interface EmployeeOnHolidayDTO extends EmployeeDTO {
	start_date: string
	end_date: string
	days_off: number
}

export class EmployeeOnHoliday extends Employee {
	startDate: Date = new Date()
	endDate: Date = new Date()
	daysOff: number = 0

	constructor(params?: Partial<Omit<EmployeeOnHoliday, 'name' | 'toCreateDTO' | 'toUpdateDTO' | 'currentlyOnHoliday'>>) {
		super(params)
		if (params?.startDate !== undefined) this.startDate = params.startDate
		if (params?.endDate !== undefined) this.endDate = params.endDate
		if (params?.daysOff !== undefined) this.daysOff = params.daysOff
	}

	get currentlyOnHoliday() {
		return this.endDate >= new Date() && this.startDate <= new Date()
	}

	static fromDTO(dto: EmployeeOnHolidayDTO): EmployeeOnHoliday {
		return new EmployeeOnHoliday({
			id: dto.id,
			firstName: dto.first_name,
			lastName: dto.last_name,
			address: dto.address,
			phone: dto.phone,
			startDate: dto.start_date ? parseDate(dto.start_date, true) : undefined,
			createdAt: parseDate(dto.created_at),
			assignments: dto.assignments?.map(EmployeeAssignment.fromDTO),
			endDate: dto.end_date ? parseDate(dto.end_date, true) : undefined,
			daysOff: dto.days_off,
		})
	}
}
