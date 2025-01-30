import { parseDate, toTitleCase } from '$utils'

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

type AssignmentObject = Partial<Omit<Assignment, 'toCreateDTO' | 'toUpdateDTO' | 'name' | 'object'>>

export class Assignment {
	id: number = 0
	taskId: number = $state(0)
	areaId: number = $state(0)
	difficulty: number = $state(1)
	frequency: string = $state('')
	createdAt: Date = new Date()
	taskName?: string = ''
	areaName?: string = ''

	constructor(params?: AssignmentObject) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.taskId !== undefined) this.taskId = params.taskId
		if (params?.areaId !== undefined) this.areaId = params.areaId
		if (params?.difficulty !== undefined) this.difficulty = params.difficulty
		if (params?.frequency !== undefined) this.frequency = params.frequency
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
		if (params?.taskName !== undefined) this.taskName = params.taskName
		if (params?.areaName !== undefined) this.areaName = params.areaName
	}

	static fromDTO(dto: AssignmentDTO): Assignment {
		return new Assignment({
			id: dto.id,
			taskId: dto.task_id,
			areaId: dto.area_id,
			difficulty: dto.difficulty,
			frequency: dto.frequency,
			createdAt: parseDate(dto.created_at),
			taskName: dto.task_name,
			areaName: dto.area_name,
		})
	}

	get name() {
		return `${this.areaName} - ${this.taskName}`
	}

	get object(): AssignmentObject {
		return {
			id: this.id,
			taskId: this.taskId,
			areaId: this.areaId,
			difficulty: this.difficulty,
			frequency: this.frequency,
			areaName: this.areaName,
			taskName: this.taskName,
			createdAt: this.createdAt,
		}
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
	SEMANAL_ODD: 'Lun, Mie, Vie',
	SEMANAL_EVEN: 'Mar, Jue',
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
