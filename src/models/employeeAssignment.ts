type CreateEmployeeAssignmentDTO = Omit<EmployeeAssignmentDTO, 'id' | 'created_at'>
type UpdateEmployeeAssignmentDTO = Omit<EmployeeAssignmentDTO, 'created_at'>

export type EmployeeAssignmentDTO = {
	id: number
	employee_id: number
	assignment_id: number
	is_primary?: boolean
	efficiency: number
	assigned_date?: string
	created_at: string
}

export class EmployeeAssignment {
	id: number
	employeeId: number
	assignmentId: number
	isPrimary?: boolean
	efficiency: number
	assignedDate?: Date
	createdAt: Date

	constructor(params?: Partial<Omit<EmployeeAssignment, 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		this.employeeId = params?.employeeId || 0
		this.assignmentId = params?.assignmentId || 0
		this.isPrimary = params?.isPrimary || false
		this.efficiency = params?.efficiency || 0
		this.assignedDate = params?.assignedDate || new Date()
		this.createdAt = params?.createdAt || new Date()
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
		})
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
