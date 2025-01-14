import { toTitleCase } from '$utils'

type UpdateAssignmentDTO = Omit<AssignmentDTO, 'created_at'>
type CreateAssignmentDTO = Omit<UpdateAssignmentDTO, 'id'>

export type AssignmentDTO = {
	id: number
	task_id: number
	area_id: number
	difficulty: number
	frequency: string
	created_at: string
	task_name?: string
	area_name?: string
}

export class Assignment {
	id: number
	taskId: number
	areaId: number
	difficulty: number
	frequency: string
	createdAt: Date
	taskName?: string
	areaName?: string

	constructor(params?: Partial<Omit<Assignment, 'toCreateDTO' | 'toUpdateDTO' | 'name'>>) {
		this.id = params?.id || 0
		this.taskId = params?.taskId || 0
		this.areaId = params?.areaId || 0
		this.difficulty = params?.difficulty || 1
		this.frequency = params?.frequency || ''
		this.createdAt = params?.createdAt || new Date()
		this.taskName = params?.taskName
		this.areaName = params?.areaName
	}

	static fromDTO(dto: AssignmentDTO): Assignment {
		return new Assignment({
			id: dto.id,
			taskId: dto.task_id,
			areaId: dto.area_id,
			difficulty: dto.difficulty,
			frequency: dto.frequency,
			createdAt: new Date(dto.created_at),
			taskName: dto.task_name,
			areaName: dto.area_name,
		})
	}

	get name() {
		return `${this.areaName} - ${this.taskName}`
	}

	public toCreateDTO(): CreateAssignmentDTO {
		return {
			task_id: this.taskId,
			area_id: this.areaId,
			difficulty: this.difficulty,
			frequency: this.frequency,
		}
	}

	public toUpdateDTO(): UpdateAssignmentDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}

export const AssignmentFrequency = {
	DIARIA: 'Diaria',
	SEMANAL: 'Semanal',
	MENSUAL: 'Mensual',
} as const
export type AssignmentFrequency = ValueOf<typeof AssignmentFrequency>
export const AssignmentFrequencies = Object.values(AssignmentFrequency).map(value => ({ label: value, value }))

export const AssignmentDifficulty = {
	TRIVIAL: 0,
	FACIL: 1,
	MEDIA: 2,
	COMPLICADO: 3,
	DIFICIL: 4,
} as const
export type AssignmentDifficulty = ValueOf<typeof AssignmentDifficulty>
export const AssignmentDifficulties = Object.entries(AssignmentDifficulty).map(([label, value]) => ({ label: toTitleCase(label), value }))
