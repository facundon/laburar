export const ROUTES = {
	employee: {
		list: '/employees',
		create: '/employees/create',
		edit: (id: number) => `/employees/${id}/edit`,
		view: (id: number) => `/employees/${id}`,
		assignTasks: (id: number) => `/employees/${id}/assign-task`,
	},
	task: {
		list: '/tasks',
		create: '/tasks/create',
		edit: (id: number) => `/tasks/${id}/edit`,
		view: (id: number) => `/tasks/${id}`,
	},
	area: {
		list: '/areas',
		create: '/areas/create',
		edit: (id: number) => `/areas/${id}/edit`,
		view: (id: number) => `/areas/${id}`,
		assignTasks: (id: number) => `/areas/${id}/assign-task`,
	},
} as const
