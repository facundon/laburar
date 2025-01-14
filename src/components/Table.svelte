<script lang="ts" generics="T extends object">
	import type { Component, Snippet } from 'svelte'

	type ColumnDef<T> = {
		[K in keyof T]: {
			field: K
			width?: number
			isBold?: boolean
			headerName: string
			formatValue?: (value: T[K]) => string
			renderCell?: (value: T[K]) => {
				component: Snippet | Component
				props: any
			}
		}
	}[keyof T]
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
					{@const value = row[column.field]}
					{@const formatedValue = column.formatValue ? column.formatValue(value) : value}
					{#if column.renderCell}
						{@const CustomCell = column.renderCell(value)}
						<td>
							<CustomCell.component {...CustomCell.props} />
						</td>
					{:else}
						<td>
							{#if column.isBold}
								<strong>{formatedValue}</strong>
							{:else}
								{formatedValue}
							{/if}
						</td>
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
		border-bottom: 1px solid #626262;
		padding: 0.25rem;
	}

	td:first-child,
	th:first-child {
		padding-left: 1rem;
	}

	td:last-child,
	th:last-child {
		padding-right: 1rem;
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
