type CreateEmployeeDTO = {
  first_name: string
  last_name: string
  address: string
  phone?: string
  start_date?: string
}

type EmployeeDTO = {
  id: number
  first_name: string
  last_name: string
  address: string
  phone?: string
  start_date?: string
  created_at: string
}

export class Employee {
  id: number
  firstName: string
  lastName: string
  address: string
  createdAt: Date
  phone?: string
  startDate?: Date

  constructor(params: Omit<Employee, "name" | "toCreateDTO">) {
    this.id = params.id
    this.firstName = params.firstName
    this.lastName = params.lastName
    this.phone = params.phone
    this.address = params.address
    this.createdAt = params.createdAt
    this.startDate = params.startDate
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
}
