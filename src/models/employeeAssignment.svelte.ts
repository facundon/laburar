import { formatDate, formatDateTime, parseDate, toTitleCase } from '$utils'
import { format } from 'date-fns'
import { SvelteDate } from 'svelte/reactivity'

type UpdateEmployeeAssignmentDTO = Omit<EmployeeAssignmentDTO, 'created_at' | 'area_id' | 'area_name' | 'task_id' | 'task_name'>
type CreateEmployeeAssignmentDTO = Omit<UpdateEmployeeAssignmentDTO, 'id'>

export type EmployeeAssignmentDTO = {
	id: number
	employee_id: number
	assignment_id: number
	is_primary?: boolean
	efficiency: number
	assigned_date?: string
	created_at: string
	area_id?: number
	area_name?: string
	task_id?: number
	task_name?: string
}

export class EmployeeAssignment {
	id: number = 0
	employeeId: number = $state(0)
	assignmentId: number = $state(0)
	isPrimary: boolean = $state(false)
	efficiency: number = $state(1)
	assignedDate: Date = new SvelteDate()
	createdAt: Date = new Date()
	areaId: number = $state(0)
	areaName: string = $state('')
	taskId: number = $state(0)
	taskName: string = $state('')

	constructor(params?: Partial<Omit<EmployeeAssignment, 'toCreateDTO' | 'toUpdateDTO' | 'name'>>) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.employeeId !== undefined) this.employeeId = params.employeeId
		if (params?.assignmentId !== undefined) this.assignmentId = params.assignmentId
		if (params?.isPrimary !== undefined) this.isPrimary = params.isPrimary
		if (params?.efficiency !== undefined) this.efficiency = params.efficiency
		if (params?.assignedDate !== undefined) this.assignedDate = params.assignedDate
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
		if (params?.areaId !== undefined) this.areaId = params.areaId
		if (params?.areaName !== undefined) this.areaName = params.areaName
		if (params?.taskId !== undefined) this.taskId = params.taskId
		if (params?.taskName !== undefined) this.taskName = params.taskName
	}

	static fromDTO(dto: EmployeeAssignmentDTO): EmployeeAssignment {
		return new EmployeeAssignment({
			id: dto.id,
			employeeId: dto.employee_id,
			assignmentId: dto.assignment_id,
			isPrimary: dto.is_primary,
			efficiency: dto.efficiency,
			assignedDate: dto.assigned_date ? new SvelteDate(dto.assigned_date) : undefined,
			createdAt: parseDate(dto.created_at),
			areaId: dto.area_id,
			areaName: dto.area_name,
			taskId: dto.task_id,
			taskName: dto.task_name,
		})
	}

	get name() {
		return `${this.areaName} - ${this.taskName}`
	}

	public toCreateDTO(): CreateEmployeeAssignmentDTO {
		return {
			employee_id: this.employeeId,
			assignment_id: this.assignmentId,
			is_primary: this.isPrimary,
			efficiency: this.efficiency,
			assigned_date: formatDateTime(this.assignedDate),
		}
	}

	public toUpdateDTO(): UpdateEmployeeAssignmentDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}

export const AssignmentEfficiency = {
	MEDIOCRE: 0,
	BAJA: 1,
	MEDIA: 2,
	ALTA: 3,
	EXCELENTE: 4,
} as const
export type AssignmentEfficiency = ValueOf<typeof AssignmentEfficiency>
export const AssignmentEfficiencies = Object.entries(AssignmentEfficiency).map(([label, value]) => ({ label: toTitleCase(label), value }))
