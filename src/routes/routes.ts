export const ROUTES = {
	employee: {
		list: '/employees',
		create: '/employees/create',
		edit: (id: number) => `/employees/${id}/edit`,
		view: (id: number) => `/employees/${id}`,
		assignTask: (id: number) => `/employees/${id}/assign-task`,
	},
	task: {
		list: '/tasks',
		create: '/tasks/create',
		edit: (id: number) => `/tasks/${id}/edit`,
		view: (id: number) => `/tasks/${id}`,
	},
} as const
