import { toTitleCase } from '$utils'

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
	id: number
	employeeId: number
	assignmentId: number
	isPrimary?: boolean
	efficiency: number
	assignedDate?: Date
	createdAt: Date
	areaId?: number
	areaName?: string
	taskId?: number
	taskName?: string

	constructor(params?: Partial<Omit<EmployeeAssignment, 'toCreateDTO' | 'toUpdateDTO' | 'name'>>) {
		this.id = params?.id || 0
		this.employeeId = params?.employeeId || 0
		this.assignmentId = params?.assignmentId || 0
		this.isPrimary = params?.isPrimary || false
		this.efficiency = params?.efficiency || 0
		this.assignedDate = params?.assignedDate || new Date()
		this.createdAt = params?.createdAt || new Date()
		this.areaId = params?.areaId || 0
		this.areaName = params?.areaName
		this.taskId = params?.taskId || 0
		this.taskName = params?.taskName
	}

	static fromDTO(dto: EmployeeAssignmentDTO): EmployeeAssignment {
		return new EmployeeAssignment({
			id: dto.id,
			employeeId: dto.employee_id,
			assignmentId: dto.assignment_id,
			isPrimary: dto.is_primary,
			efficiency: dto.efficiency,
			assignedDate: dto.assigned_date ? new Date(dto.assigned_date) : undefined,
			createdAt: new Date(dto.created_at),
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
			assigned_date: this.assignedDate?.toISOString(),
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
