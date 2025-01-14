<script lang="ts" generics="T extends object">
	import type { Component, Snippet } from 'svelte'

	interface ColumnDef<T> {
		field: keyof T
		width?: number
		isBold?: boolean
		headerName: string
		renderCell?: (row: T) => {
			component: Snippet | Component
			props: any
		}
	}
	interface Props<T extends object> {
		columns: ColumnDef<T>[]
		rows: T[]
	}

	let { columns, rows }: Props<T> = $props()
</script>

<table>
	<thead>
		<tr>
			{#each columns as column}
				<th>{column.headerName}</th>
			{/each}
		</tr>
	</thead>
	<tbody>
		{#each rows as row}
			<tr>
				{#each columns as column}
					{#if column.renderCell}
						{@const CustomCell = column.renderCell(row)}
						<td>
							<CustomCell.component {...CustomCell.props} />
						</td>
					{:else if column.isBold}
						<td><strong>{row[column.field]}</strong></td>
					{:else}
						<td>{row[column.field]}</td>
					{/if}
				{/each}
			</tr>
		{/each}
	</tbody>
</table>

<style>
	table {
		width: 100%;
		border-collapse: collapse;
	}

	th,
	td {
		padding: 0.5rem;
		border-bottom: 1px solid #626262;
	}

	th {
		text-align: left;
	}

	tr:last-child td {
		border-bottom: none;
	}

	tr:nth-child(even) {
		background-color: #f2f2f2d7;
	}
	tr:nth-child(odd) {
		background-color: #f2f2f2;
	}

	th {
		background-color: var(--primary-main);
		color: var(--primary-contrast);
	}

	td {
		color: #333;
	}
</style>
