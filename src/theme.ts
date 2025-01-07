export const theme = {
	colors: {
		primary: {
			main: '#FED766', // Mustard
			light: '#FFE599', // Lighter shade of mustard
			dark: '#D4A53A', // Darker shade of mustard
			contrast: '#333',
		},
		secondary: {
			main: '#59C3C3', // Blue
			light: '#7AD1D1', // Lighter shade of main
			dark: '#3F8A8A', // Darker shade of main
			contrast: '#333',
		},
		success: {
			main: '#72A98F', // Green
			light: '#A3D1B8', // Lighter shade of green
			dark: '#4A7A63', // Darker shade of green
			contrast: '#FBFBFF',
		},
		error: {
			main: '#ED6A5A', // Red
			light: '#F29B8E', // Lighter shade of red
			dark: '#B94A3D', // Darker shade of red
			contrast: '#FBFBFF',
		},
		gray: {
			main: '#343a40', // Gray for the menu background
			light: '#495057',
			dark: '#212529',
			contrast: '#FBFBFF',
		},
	},
}

export const cssVariables = `
		--primary-main: ${theme.colors.primary.main};
		--primary-light: ${theme.colors.primary.light};
		--primary-dark: ${theme.colors.primary.dark};
		--primary-contrast: ${theme.colors.primary.contrast};
		--secondary-main: ${theme.colors.secondary.main};
		--secondary-light: ${theme.colors.secondary.light};
		--secondary-dark: ${theme.colors.secondary.dark};
		--secondary-contrast: ${theme.colors.secondary.contrast};
		--success-main: ${theme.colors.success.main};
		--success-light: ${theme.colors.success.light};
		--success-dark: ${theme.colors.success.dark};
		--success-contrast: ${theme.colors.success.contrast};
		--error-main: ${theme.colors.error.main};
		--error-light: ${theme.colors.error.light};
		--error-dark: ${theme.colors.error.dark};
		--error-contrast: ${theme.colors.error.contrast};
		--gray-main: ${theme.colors.gray.main};
		--gray-light: ${theme.colors.gray.light};
		--gray-dark: ${theme.colors.gray.dark};
		--gray-contrast: ${theme.colors.gray.contrast};
`
