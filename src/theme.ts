export const theme = {
	colors: {
		primary: {
			main: '#FED766', // Mustard
			light: '#FFE599', // Lighter shade of mustard
			dark: '#D4A53A', // Darker shade of mustard
			contrast: '#000000',
		},
		secondary: {
			main: '#655A7C', // Purple
			light: '#8A7FA3', // Lighter shade of purple
			dark: '#4A3F5A', // Darker shade of purple
			contrast: '#FBFBFF',
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
