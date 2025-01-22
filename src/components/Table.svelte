<script lang="ts" generics="T extends object, P extends Record<string, any>">
	import type { Component } from 'svelte'
	import type { IconProps, Icon as IconType } from 'lucide-svelte'

	type RenderIcon = {
		component: typeof IconType
		props: IconProps
	}

	type RenderComponent<P extends Record<string, any>> = {
		component: Component<P>
		props: P
	}

	type ColumnDef<T, P extends Record<string, any>> = {
		[K in keyof T]: {
			field: K
			width?: number
			isBold?: boolean
			headerName: string
			formatValue?: (value: T[K]) => string
			renderCell?: (value: T[K]) => RenderComponent<P> | RenderIcon
		}
	}[keyof T]
	interface Props<T extends object, P extends Record<string, any>> {
		columns: ColumnDef<T, P>[]
		rows: T[]
		highlightIndex?: number
	}

	let { columns, rows, highlightIndex = -1 }: Props<T, P> = $props()
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
		{#if rows.length === 0}
			<tr>
				<td colspan={columns.length} class="missing-row">No hay datos</td>
			</tr>
		{/if}
		{#each rows as row, index}
			<tr class:highlighted={index < highlightIndex}>
				{#each columns as column}
					{@const value = row[column.field]}
					{@const formatedValue = column.formatValue ? column.formatValue(value) : value}
					{#if column.renderCell}
						{@const CustomCell = column.renderCell(value)}
						<td width={column.width}>
							<CustomCell.component {...CustomCell.props as P} />
						</td>
					{:else}
						<td width={column.width}>
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
	.missing-row {
		text-align: center;
		padding: 3rem;
	}
	table {
		width: 100%;
		border-collapse: collapse;
	}

	th,
	td {
		border-bottom: 1px solid #626262;
		padding-inline: 1rem;
		padding-block: 0.25rem;
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
	tr {
		background-color: #f2f2f2;
	}

	/* tr:nth-child(even) {
		background-color: #f2f2f2d7;
	}
	tr:nth-child(odd) {
		background-color: #f2f2f2;
	} */

	th {
		background-color: var(--primary-main);
		color: var(--primary-contrast);
	}

	td {
		color: #333;
	}

	.highlighted {
		background-color: #e0e0e0;
	}
</style>
